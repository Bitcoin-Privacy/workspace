-- drop table if exists room, txin, txout, proof; 
create extension if not exists "uuid-ossp";

create table if not exists room (
  id uuid default uuid_generate_v1() not null constraint rooms_pkey primary key,
  no_peer int2 default 0 not null,
  status int2 default 0 not null,
  base_amount int8 not null,
  due1 int8 not null,
  due2 int8 not null,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp,
  constraint chk_status check (status in (0, 1, 2, 3, 4))
);

create table if not exists txin (
  id uuid default uuid_generate_v1() not null constraint txins_pkey primary key,
  room_id uuid,
  txid varchar(64),
  vout int4,
  amount int8,
  created_at timestamp with time zone not null default current_timestamp,
  foreign key (room_id) references room (id)
);

create table if not exists txout (
	id uuid default uuid_generate_v1() not null constraint txouts_pkey primary key,
	room_id uuid,
	address varchar,
	amount int8,
	created_at timestamp with time zone not null default current_timestamp,
	foreign key (room_id) references room (id)
);

create table if not exists proof (
	id uuid default uuid_generate_v1() not null constraint proofs_pkey primary key,
	room_id uuid,
	vin int4,
	script varchar,
	created_at timestamp with time zone not null default current_timestamp,
	foreign key (room_id) references room (id)
);

create or replace function
    add_new_peer(
        p_room_id uuid,
        p_txid varchar(64),
        p_vout int,
        p_amount int8,
        p_change int8,
        p_address varchar
    )
returns void
as $$
begin
    -- insert into txin table
    insert into txin (room_id, txid, vout, amount)
    values (p_room_id, p_txid, p_vout, p_amount);

    -- insert into txout table
    insert into txout (room_id, amount, address)
    values (p_room_id, p_change, p_address);

    -- update the no_peer count in room table
    update room
    set no_peer = no_peer + 1
    where id = p_room_id;
exception
    WHEN unique_violation THEN
        RAISE NOTICE 'A unique violation occurred.';
    when others then
        -- in case of an error, rollback the transaction
        raise notice 'An error occurred: %', SQLERRM;
end;
$$
language plpgsql
;
