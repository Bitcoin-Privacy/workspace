create table if not exists Account (
  id TEXT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  url TEXT NOT NULL,
  deriv TEXT NOT NULL
);


create table if not exists Config (
  key TEXT NOT NULL PRIMARY KEY,
  value TEXT NOT NULL
);

create table if not exists Utxo (
  id TEXT NOT NULL PRIMARY KEY,
  address TEXT NOT NULL,
  txid TEXT NOT NULL,
  vout INTEGER NOT NULL,
  value INTEGER NOT NULL
);

create table if not exists CoinJoinUtxo (
  utxo_id TEXT NOT NULL PRIMARY KEY,
  room_id TEXT NOT NULL
);

create table if not exists CoinJoinRoom (
  id TEXT NOT NULL PRIMARY KEY,

  base_amount INTEGER NOT NULL,
  num_of_peer INTEGER NOT NULL,
  status INTEGER NOT NULL,

  due1 INTEGER NOT NULL,
  due2 INTEGER NOT NULL,

  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Statecoin (
    statechain_id TEXT NOT NULL PRIMARY KEY,
    account_id TEXT,
    token_id TEXT,
    signed_statechain_id TEXT,
    amount INT,
    server_pubkey_share BLOB,
    aggregated_pubkey BLOB,
    p2tr_agg_address TEXT,

    funding_txid TEXT,
    funding_vout INT,

    status TEXT,
    locktime INT,

    client_pubkey_share BLOB,

    tx_withdraw TEXT,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (account_id) REFERENCES Account(id)
);

CREATE TABLE IF NOT EXISTS BackupTransaction (
    id TEXT NOT NULL PRIMARY KEY,
    tx_n INT,
    statechain_id TEXT,
    client_public_nonce BLOB,
    server_public_nonce BLOB,
    client_pubkey BLOB,
    server_pubkey BLOB,
    blinding_factor BLOB,
    backup_tx BLOB,
    recipient_address TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (statechain_id) REFERENCES Statecoin(statechain_id)
);
