# Auth Service

Generic authentication and authorization service based on Ethereum wallets

### Mechanism

1. Server asks the client to sign a hash with the private key
2. Client signs the hash and sends it back alongside with the public key
3. Server returns a token signed by the server.
4a. Client send this to the app 
4b. Apps can introspect the token from the server

### Contribution

```
$ docker-compose up -d
$ docker exec -it cockroachdb ./cockroach sql --insecure
> CREATE DATABASE auth_service;

$ diesel migration run # install diesel_cli
```
