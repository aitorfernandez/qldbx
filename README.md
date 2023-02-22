# qldbx

`qldbx` is a Rust package that simplifies the process of interacting with Amazon Quantum Ledger Database (QLDB) by providing a collection of commands to create, delete, reset, and run migrations.

## Basic Configuration

`qldbx` needs to set up their AWS security credentials and configuration before running the package commands.

Set up credentials (in e.g. ~/.aws/credentials):

```
[test]
aws_access_key_id=<access key id>
aws_secret_access_key=<secret key>
```

Set up a default region (in e.g. ~/.aws/config):

```
[profile test]
region=<region>
```

Export the profile:

```
export AWS_PROFILE=test
```

Set up the `.env` file with the ledger name and ledger uri:

```
URI=<ledger uri>
NAME=<ledger name>
```

## Available commands

Create a new ledger.

```
cargo run ledger create
```

Delete an existing ledger.

```
cargo run ledger delete
```

Reset the ledger by deleting and running all migrations.

```
cargo run ledger reset
```

Create a migration empty file.

```
cargo run migrate create <migration name>
```

Run all created migrations.

```
cargo run ledger run
```

Display information about the migrations applied and not applied.

```
cargo run ledger info
```