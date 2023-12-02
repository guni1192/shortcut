#!/usr/bin/env bash

set -eux

source .env

docker compose build
docker compose up -d

cargo install sqlx-cli --no-default-features --features rustls,postgres
cargo sqlx migrate run

cargo test --tests
