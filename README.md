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

`POST /auth/login`

Main Login/Sign in endpoint. After the login there is the `token` field in the response. It should be used for authentication.

#### Request

- `eth_address`: The Ethereum address to sign in.
- `signature`: The signature with Ethereum wallet or with any private key belongs to the Ethereum address.

#### Response

- Returns a [User](#user) with the information of the user signed in.

#### Example

```
// Request
curl --location --request POST 'localhost:8000/auth/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "eth_address":"HAc5Hnb52AtBnght5T5G7jYyMeMbcVIFv",
    "signature": "82iAGsEJWQSK0i4Te0ruK283o8817PtR24MN1v273EQwMOJSSNaHwRoD5dj2QC5Owt67uH53EaC8Lnw5Ivou7T20Dtx6c14GC1Ui4qhQEPhJ5811EMJ2DsF2oQB2ruMl"
}'

// Response
200 OK
{
    "user_id": "caea8aee-9b49-4f6d-99bc-3c97c38667f1",
    "groups": {
        "e68e886b-b2a4-45c1-b983-c47549738766": {
            "name": "D6C6safE7ta",
            "permissions": {
                "557a3071-e0b8-4fac-b350-be121b13f12d": {
                    "name": "uH0uA5_WRITE"
                }
            }
        }
    },
    "internal_permissions": [
        "manage_permissions",
        "manage_users",
        "manage_groups",
        "manage_tokens",
        "set_internal_permissions",
        "get_users",
        "get_groups",
        "get_permissions"
    ],
    "eth_address": "HAc5Hnb52AtBnght5T5G7jYyMeMbcVIFv",
    "token": {
        "token": "e23fa9ba-1fe4-4ab5-9424-be33c84079cf",
        "expires_at": "2021-06-09T20:55:49.137387+00:00",
        "valid": true
    }
}
```

### Create register token endpoint

`POST /auth/tokens/register`

Register token required for registration. So one of the member of the team can create a register token and send to the user securely, so it can register.
It requires to have `manage_users` internal permission.

#### Headers

- `Authorization`: `{token_from_login_response}`

#### Response

- `token`: Register token, this should be sent to the user.
- `expires_at`: Time when the token expires.
- `valid`: Boolean which always `true` in this case.

#### Example

```
// Request
curl --location --request POST 'localhost:8000/auth/tokens/register' \
--header 'Authorization: e23fa9ba-1fe4-4ab5-9424-be33c84079cf'

// Response
200 OK
{
    "token": "4a278b0c-a00f-4bef-bb4c-d34b97a2b5ac",
    "expires_at": "2021-06-09T20:57:56.696567+00:00",
    "valid": true
}
```

### Register endpoint

`POST /auth/register`

After the user get the [register token](#create-register-token-endpoint) they can register. It can be done by sending the eth_address, and the signature alongside with the register token.
Note, that the register is very strict, the first action is always to delete the register token even if the registration NOT finished properly.
After registration the system automatically log in the user.

#### Request

- `eth_address`: The Ethereum address to sign in.
- `signature`: The signature with Ethereum wallet or with any private key belongs to the Ethereum address.
- `register_token`: The register token acquired by a member who has permission for that.

#### Response

- Returns a [User](#user) with the information of the user signed in.

#### Example

```
// Request
curl --location --request POST 'localhost:8000/auth/register' \
--header 'Content-Type: application/json' \
--data-raw '{
    "eth_address": "test1234123123123123123",
    "signature": "test1234123123123123123",
    "register_token":"02a387f2-e1b0-4437-a12e-e901d8d9373b"
}'

// Response 
200 OK
{
    "user_id": "222e3bf3-7900-48e1-9e96-bf4a16562ab7",
    "groups": {},
    "internal_permissions": [],
    "eth_address": "test1234123123123123123",
    "token": {
        "token": "54acf92e-247d-49d7-b2ae-955a678c9436",
        "expires_at": "2021-06-10T18:49:00.163140+00:00",
        "valid": true
    }
}
```

### Introspection

`GET /auth/introspection`

Introspection is a process when we request information about a certain token.

#### Headers

- `Authorization`: `{token_from_login_response}`

#### Response

- Returns a [User](#user) with the information of the token's owner.

#### Example

```
// Request
curl --location --request GET 'localhost:8000/auth/introspection' \
--header 'Authorization: 54acf92e-247d-49d7-b2ae-955a678c9436'

// Response
200 OK
{
    "user_id": "222e3bf3-7900-48e1-9e96-bf4a16562ab7",
    "groups": {},
    "internal_permissions": [],
    "eth_address": "test1234123123123123123",
    "token": {
        "token": "54acf92e-247d-49d7-b2ae-955a678c9436",
        "expires_at": "2021-06-10T18:49:00.163140+00:00",
        "valid": true
    }
}
```

### Create permission

`POST /auth/permissions`

Create permission endpoint. Name should be unique.
It requires to have `manage_permissions` internal permission.

#### Headers

- `Authorization`: `{token_from_login_response}`

#### Request

- `name`: The name of the permission.

#### Response 

- `id`: ID of the new permission.
- `name`: Name added by the creator.
- `created_at`: Timestamp of the creation.

#### Example

```
// Request
curl --location --request POST 'localhost:8000/auth/permissions' \
--header 'Authorization: c9e8b521-67c5-4088-82e4-16cff94ff40e' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "test_WRITE"
}'

// Response
200 OK
{
    "id": "23d0a33a-418e-413c-b45f-7c220deec16f",
    "name": "test_WRITE",
    "created_at": ...,
}
```

### Create group

`POST /auth/groups`

Create group endpoint. Name should be unique.
It requires to have `manage_groups` internal permission.

#### Headers

- `Authorization`: `{token_from_login_response}`

#### Request

- `name`: The name of the group.
- `description`: The description of the group.

#### Response

- `id`: ID of the new group.
- `name`: Name added by the creator.
- `description`: Description added by the creator.
- `created_at`: Timestamp of the creation.

#### Example

```
// Request
curl --location --request POST 'localhost:8000/auth/groups' \
--header 'Authorization: c9e8b521-67c5-4088-82e4-16cff94ff40e' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "test_WRITE",
    "description": "shit Java"
}'

// Response
200 OK
{
    "id": "af03c47b-7bb2-476b-9ec6-0ec45af2a1c2",
    "name": "test_WRITE",
    "description": "shit Java",
    "created_at": {
        "secs_since_epoch": 1623341618,
        "nanos_since_epoch": 50303000
    },
    "deleted_at": null
}
```

----------------------------------------------------------------

### Common structures

#### User

- `user_id`: The internal ID of the user.
- `groups`: Hash map of the groups where the user member of. Key is the group's ID and the value is [Group](#group).
- `internal_permissions`: List of internal permissions of the user.
- `eth_address`: The Ethereum address of the user who signed in.
- `token`: Token used for authentication.

#### Group

- `name`: The human-readable name of the group.
- `permissions`: Hash map of the permissions what user has in particular group. Key is the permission's ID and the value is [Permission](#permission).

#### Permission

- `name`: The human-readable name of the permission.


### Contribution

```
$ docker-compose up -d
$ docker exec -it cockroachdb ./cockroach sql --insecure
> CREATE DATABASE auth_service;

$ diesel migration run # install diesel_cli
```
