#!/usr/bin/env bash


docker compose build
docker compose run api-test cargo test
