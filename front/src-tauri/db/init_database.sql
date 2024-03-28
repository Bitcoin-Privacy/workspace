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
