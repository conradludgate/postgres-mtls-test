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

### Explanation

Certificates:
* `root.pem` is the root CA certificate
* `server.pem`/`server.key` is the server certificate key pair, signed by the root CA. Has the common name `ep-foo-bar-1234.eu-west-1.aws.
* `proxy.pem` is an intermediate CA certificate, signed by the root CA. Has the common name `proxy.eu-west-1.aws.neon.build`
* `user1.pem`/`user1.key` is a client certificate, signed by the proxy CA. Has the common name `user1@ep-foo-bar-1234.proxy.eu-west-1.aws.neon.build`
* `user2.pem`/`user2.key` is a client certificate, signed by the proxy CA. Has the common name `user2@ep-foo-baz-9876.proxy.eu-west-1.aws.neon.build`

pg_hba.conf:
```
# TYPE  DATABASE        USER            ADDRESS                 METHOD

# local logins are trusted
local   all             all                                     trust

# connections over TCP require SSL and must provide a client certificate.
hostssl all             all             all                     cert     clientcert=verify-full map=proxy
```

pg_ident.conf:
```
# map users only if they have the common name `*@ep-foo-bar-1234.proxy.eu-west-1.aws.neon.build`
# which will only accept user1.pem and not user2.pem.
proxy   /^(.*)@ep-foo-bar-1234.proxy.eu-west-1.aws.neon.build$   \1
```
