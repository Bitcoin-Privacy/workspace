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

CREATE TABLE IF NOT EXISTS StateCoin (
    --statecoin info
    statechain_id TEXT NOT NULL PRIMARY KEY,
    signed_statechain_id TEXT,
    deriv TEXT NOT NULL,
    amount INT,
    tx_n INT ,
    n_lock_time INT,

    --key pairs
    key_agg_ctx TEXT,
    aggregated_pubkey TEXT,
    aggregated_address TEXT,
    auth_pubkey INT,
    auth_seckey INT, 
    owner_pubkey TEXT,
    owner_seckey TEXT,
    --deposit tx info
    funding_txid TEXT,
    funding_vout INT,
    funding_tx TEXT,
  

    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create table if not exists TransferStatecoin (
  account TEXT PRIMARY KEY,
  auth_pubkey INT,
  auth_seckey INT, 
  owner_pubkey TEXT,
  owner_seckey TEXT


  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS BackupTransaction (
    id INTEGER,
    tx_n INT ,
    n_lock_time INT,
    statechain_id TEXT,
    backup_tx  TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY("id" AUTOINCREMENT),
    FOREIGN KEY (statechain_id) REFERENCES Statecoin(statechain_id)
);
