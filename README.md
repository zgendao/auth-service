# Auth Service

Generic authentication and authorization service based on Ethereum wallets

## Login Mechanism

1. Server asks the client to sign a hash with the private key
2. Client signs the hash and sends it back alongside with the public key
3. Server returns a token signed by the server.
4a. Client send this to the app 
4b. Apps can introspect the token from the server

## Endpoints

### Login endpoint 

`POST /login`

#### Request

- `eth_address`: The Ethereum address to sign in.
- `signature`: The signature with Ethereum wallet or with any private key belongs to the Ethereum address.

#### Response

- Returns a [User](#user) with the information of the user signed in.

### Common structures

#### User

- `user_id`: The internal ID of the user.
- `groups`: Hash map of the groups where the user member of. Key is the group's ID and the value is [Group](#group).
- `internal_permissions`: List of internal permissions of the user.
- `eth_address`: The Ethereum address of the user who signed in.
- `token`: Token used for authentication.

#### Group

- `name`: The human-readable name of the group.
- `permissions`: Hash map of the permissions what user has in particular group. Key is the permission's ID and the value is [Permission](#permission)

#### Permission

- `name`: The human-readable name of the permission.


### Contribution

```
$ docker-compose up -d
$ docker exec -it cockroachdb ./cockroach sql --insecure
> CREATE DATABASE auth_service;

$ diesel migration run # install diesel_cli
```
