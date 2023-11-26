#!/usr/bin/env bash

set -eu

docker compose build
docker compose up -d
docker compose run api-test
