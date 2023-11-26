#!/usr/bin/env bash

set -eux

docker compose build
docker compose run api-test
