# Prepare Environment

## Install sqlx cli

```console
cargo install sqlx-cli
```

## Boot environment

```
docker compose up -d
```

## Create Database

```
export DATABASE_URL=mysql://shortcut:password@localhost:3306/shortcut
cargo sqlx database create
```

## Database migration

```
cargo sqlx migrate run
```
