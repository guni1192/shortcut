#!/usr/bin/env bash
set -eux

cargo install sqlx-cli --no-default-features --features rustls,mysql
# cargo sqlx prepare
# cargo sqlx database create
cargo sqlx migrate run
