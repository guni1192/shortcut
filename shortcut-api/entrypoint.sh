#!/usr/bin/env bash
set -eux

cargo install sqlx-cli --no-default-features --features rustls,mysql
cargo sqlx migrate run
