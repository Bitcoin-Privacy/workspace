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
    deriv TEXT NOT NULL,
    amount INT,

    --key pairs
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
    status TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS BackupTransaction (
    id TEXT NOT NULL PRIMARY KEY,
    tx_n INT,
    statechain_id TEXT,
    client_pubnonce TEXT,
    server_pubnonce  TEXT,
    client_pubkey  TEXT,
    server_pubkey  TEXT,
    blinding_factor  TEXT,
    backup_tx  TEXT,
    recipient_address TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (statechain_id) REFERENCES Statecoin(statechain_id)
);
