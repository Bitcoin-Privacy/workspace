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
    --statecoin info
    id INTEGER,
    statechain_id TEXT UNIQUE ,
    signed_statechain_id TEXT,
    account TEXT NOT NULL,
    tx_n INT ,
    n_lock_time INT,
    amount INT,

    --key pairs
    key_agg_ctx TEXT,
    aggregated_pubkey TEXT,
    aggregated_address TEXT,

    auth_pubkey TEXT UNIQUE,
    auth_seckey TEXT, 
    owner_pubkey TEXT,
    owner_seckey TEXT,
    bk_tx TEXT,
    --deposit tx info
    funding_txid TEXT,
    funding_vout INT,
    funding_tx TEXT,
    isVerified boolean NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY("id" AUTOINCREMENT)
);

-- create table if not exists TransferStatecoin (
--   id INTEGER,
--   account TEXT,
--   auth_pubkey UNIQUE TEXT,
--   auth_seckey TEXT, 
--   owner_pubkey TEXT ,
--   owner_seckey TEXT,
--   created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
--   PRIMARY KEY("id" AUTOINCREMENT)
-- );

-- CREATE TABLE IF NOT EXISTS BackupTransaction (
--     --authkey TEXT PRIMARY KEY,
--     id INTEGER,
--     tx_n INT ,
--     n_lock_time INT,
--     statechain_id TEXT UNIQUE,
--     backup_tx  TEXT,
--     created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
--     PRIMARY KEY("id" AUTOINCREMENT),
--     FOREIGN KEY (statechain_id) REFERENCES StateCoin(statechain_id)
-- );
