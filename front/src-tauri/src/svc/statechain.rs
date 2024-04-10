use anyhow::{anyhow, Result};
use bitcoin::{
    absolute, consensus,
    key::{Keypair, TapTweak, TweakedKeypair},
    secp256k1::{rand, schnorr::Signature, PublicKey, Secp256k1, SecretKey},
    sighash::{Prevouts, SighashCache},
    transaction::Version,
    Address, Amount, EcdsaSighashType, Network, OutPoint, ScriptBuf, Sequence, TapSighashType,
    Transaction, TxIn, TxOut, Txid, Witness,
};

use secp256k1::Message;
use tokio::sync::OwnedMutexGuard;

use std::{num::ParseIntError, ops::ControlFlow, str::FromStr};

use crate::{api::statechain, cfg::BASE_TX_FEE, db::PoolWrapper, model::AccountActions};
use shared::intf::statechain::{AggregatedPublicKey, DepositReq, DepositRes};

use crate::connector::NodeConnector;

use super::account;
use curve25519_dalek::scalar::Scalar;

pub async fn deposit(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    deriv: &str,
    amount: u64,
) -> Result<AggregatedPublicKey> {
    let secp = Secp256k1::new();
    // let keypair = Keypair::new(&secp, &mut rand::thread_rng());
    // let xonly_pubkey = XOnlyPublicKey::from_keypair(&keypair).0;

    let auth_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let auth_seckey = SecretKey::from_keypair(&auth_keypair);
    let auth_pubkey = PublicKey::from_keypair(&auth_keypair);

    let (account, _) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();
    let req = DepositReq {
        token_id: "abc".to_string(),
        addr: auth_pubkey.to_string(),
        amount: amount as u32,
    };
    println!("Deposit request {:#?}", req);
    let body = serde_json::to_value(req)?;
    let res = conn.post("statechain/deposit", &body).await?;

    let json: DepositRes = serde_json::from_value(res)?;
    println!("Deposit response {:#?}", json);
    // response
    let se_pubkey = json.se_pubkey_1;
    let statechain_id = json.statechain_id;

    //gen o1
    let owner_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let owner_seckey = SecretKey::from_keypair(&owner_keypair);
    let owner_pubkey = PublicKey::from_keypair(&owner_keypair);

    //gen auth_key

    // combine 2 address
    let key =
        create_aggregated_address(owner_pubkey.to_string(), se_pubkey, Network::Testnet).unwrap();
    if let Err(e) = pool
        .insert_statecoin(
            &statechain_id,
            &account_address,
            amount,
            &auth_seckey,
            &auth_pubkey,
            &key.aggregated_pubkey,
            &key.aggregated_address,
            &owner_seckey,
            &owner_pubkey,
        )
        .await
    {
        panic!("Failed to insert statecoin data {:?}", e);
    }
    let txid = create_deposit_transaction(
        &pool,
        &deriv,
        amount,
        &key.aggregated_address,
        &statechain_id,
    )
    .await?;

    let tx = create_bk_tx(
        &pool,
        &conn,
        &key.aggregated_pubkey,
        &key.aggregated_address,
        &account_address,
        &txid,
        1,
        amount,
        &statechain_id,
    )
    .await
    .unwrap();
    println!("bk tx : {}", consensus::encode::serialize_hex(&tx));

    Ok(key)
}

pub fn create_aggregated_address(
    k1: String,
    k2: String,
    network: Network,
) -> Result<AggregatedPublicKey> {
    let secp = Secp256k1::new();
    let pub_k1 = PublicKey::from_str(&k1)?;
    let pub_k2 = PublicKey::from_str(&k2)?;

    let aggregated_pubkey = pub_k1.combine(&pub_k2)?;

    let aggregated_address = Address::p2tr(
        &secp,
        aggregated_pubkey.x_only_public_key().0,
        None,
        network,
    );

    Ok(AggregatedPublicKey {
        aggregated_pubkey: aggregated_pubkey.to_string(),
        aggregated_address: aggregated_address.to_string(),
    })
}

pub async fn create_deposit_transaction(
    pool: &PoolWrapper,
    deriv: &str,
    amount: u64,
    aggregated_address: &str,
    statechain_id: &str,
) -> Result<String> {
    let (account, mut unlocker) = account::get_account(deriv).unwrap();
    let selected_utxos = account::get_utxos_set(&account.get_addr(), amount).await?;

    let mut fee: u64 = 0;
    let input: Vec<TxIn> = selected_utxos
        .iter()
        .map(|utxo| {
            fee += utxo.value;
            println!("utxos set :{}", utxo.value);
            TxIn {
                previous_output: OutPoint::new(utxo.txid.parse().unwrap(), utxo.vout.into()),
                script_sig: ScriptBuf::from_bytes(vec![]),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }
        })
        .collect();

    let mut output: Vec<TxOut> = Vec::new();
    let (change, overflow) = fee.overflowing_sub(amount + BASE_TX_FEE);
    if overflow {
        return Err(anyhow!("Total amount cannot cover amount and fee"));
    }
    if change > 0 {
        output.push(TxOut {
            value: Amount::from_sat(change),
            script_pubkey: account.get_checked_addr().script_pubkey(),
        });
    }

    let addr = Address::from_str(aggregated_address).unwrap();
    let checked_addr = addr.require_network(Network::Testnet).unwrap();

    output.push(TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: checked_addr.script_pubkey(),
    });

    let deposit_tx = Transaction {
        version: Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: input,
        output: output,
    };

    let mut unsigned_deposit_tx = deposit_tx.clone();

    let secp = Secp256k1::new();
    let sighash_type = EcdsaSighashType::All;
    let mut sighasher = SighashCache::new(&mut unsigned_deposit_tx);

    let future_tasks: Vec<_> = deposit_tx
        .input
        .iter()
        .enumerate()
        .map(|(index, input)| {
            tokio::spawn(tokio::spawn(account::find_and_join_txn(
                index,
                input.clone(),
            )))
        })
        .collect();

    let mut results = Vec::new();
    for job in future_tasks {
        results.push(job.await.unwrap().unwrap().unwrap());
    }

    let res = results.iter().try_for_each(|(index, input, tx)| {
        match account::sign(
            &secp,
            &mut sighasher,
            sighash_type,
            &account,
            &mut unlocker,
            index,
            input,
            tx,
        ) {
            Ok(_) => ControlFlow::Continue(()),
            Err(e) => ControlFlow::Break(e),
        }
    });
    if let ControlFlow::Break(e) = res {
        return Err(e);
    }

    let tx_hex = consensus::encode::serialize_hex(&unsigned_deposit_tx);
    println!("hash: {:?}", tx_hex);
    println!("{:#?}", unsigned_deposit_tx);
    let funding_txid = unsigned_deposit_tx.txid().to_string();
    let funding_vout = 1 as u64;
    let _ = pool
        .update_deposit_tx(
            &statechain_id,
            &funding_txid,
            funding_vout,
            "CONFIRM",
            &tx_hex,
        )
        .await?;

    Ok(funding_txid)
}

pub async fn create_bk_tx(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    agg_pubkey: &str,
    agg_address: &str,
    receiver_address: &str,
    txid: &str,
    vout: u32,
    amount: u64,
    statechain_id: &str,
) -> Result<Transaction> {
    let agg_addr = Address::from_str(&agg_address).unwrap();
    let checked_agg_addr = agg_addr.require_network(Network::Testnet).unwrap();
    let agg_scriptpubkey = checked_agg_addr.script_pubkey();

    let utxo = TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: checked_agg_addr.script_pubkey(),
    };

    let prev_outpoint = OutPoint {
        txid: Txid::from_str(txid).unwrap(),
        vout: vout,
    };

    let input = TxIn {
        previous_output: prev_outpoint,
        script_sig: ScriptBuf::default(),
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        witness: Witness::default(),
    };

    let output_address = Address::from_str(receiver_address).unwrap();
    let checked_output_address = output_address.require_network(Network::Testnet).unwrap();
    let spend = TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: checked_output_address.script_pubkey(),
    };

    let mut unsigned_tx = Transaction {
        version: Version::TWO,               // Post BIP-68.
        lock_time: absolute::LockTime::ZERO, // Ignore the locktime.
        input: vec![input],                  // Input goes into index 0.
        output: vec![spend],                 // Outputs, order does not matter.
    };

    // request signature from server
    let res = statechain::request_sign_bk_tx(
        &conn,
        &statechain_id,
        &consensus::encode::serialize_hex(&unsigned_tx),
        &agg_scriptpubkey.to_hex_string(),
    )
    .await?;
    print!("server sign bk: {}", res.sig);

    let server_sig = Signature::from_str(&res.sig).unwrap();

    // bee4638722356ded164fa78c66933f903af20672933ac49ed10305559e39ab2eb5ef3b7bb79852fdc5402ce5feefff45a63ad017648d791ff01451780c06ddf7

    let input_index = 0;

    let sighash_type = TapSighashType::Default;
    let prevouts = vec![utxo];
    let prevouts = Prevouts::All(&prevouts);

    let mut sighasher = SighashCache::new(&mut unsigned_tx);
    let sighash = sighasher
        .taproot_key_spend_signature_hash(input_index, &prevouts, sighash_type)
        .expect("failed to construct sighash");

    let secp = Secp256k1::new();
    let seckey = pool
        .get_seckey_by_id(&statechain_id)
        .await
        .unwrap()
        .unwrap();
    let seckey = SecretKey::from_str(&seckey).unwrap();
    let keypair = Keypair::from_secret_key(&secp, &seckey);

    let tweaked: TweakedKeypair = keypair.tap_tweak(&secp, None);
    let msg = Message::from(sighash);

    let owner_sig = secp.sign_schnorr(&msg, &tweaked.to_inner());

    // TO DO : Sign = owner_sign + server_sign

    Ok(unsigned_tx)
}
