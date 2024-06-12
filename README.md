# postgres-mtls-test

Test with mTLS for Neon with PostgreSQL.

## Demo

### Setup

```sh
# setup ssl certs
mkdir certs
cargo run
chmod 600 certs/*

# init the postgres database
export PGDATA="$PWD/data"
initdb --set include="$PWD/config/ssl.conf" \
    --set hba_file="$PWD/config/pg_hba.conf" \
    --set ident_file="$PWD/config/pg_ident.conf"

# start the postgres database
pg_ctl start --log data/log.txt

psql "dbname=postgres" -c "create user user1; create user user2"
```

### Run

```sh
# connect to the db
psql "port=5432 host=localhost user=user1 dbname=postgres sslcert=./certs/user1_chain.pem sslkey=./certs/user1.key sslrootcert=./certs/root.pem sslmode=verify-ca" -c "select CURRENT_USER"

# connect to the db with the wrong user
psql "port=5432 host=localhost user=user2 dbname=postgres sslcert=./certs/user1_chain.pem sslkey=./certs/user1.key sslrootcert=./certs/root.pem sslmode=verify-ca"

# connect to the db with the wrong cert
psql "port=5432 host=localhost user=user2 dbname=postgres sslcert=./certs/user2_chain.pem sslkey=./certs/user2.key sslrootcert=./certs/root.pem sslmode=verify-ca"
```
