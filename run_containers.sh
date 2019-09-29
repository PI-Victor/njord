#!/usr/bin/env bash

docker network rm njord || true
docker network create --subnet 10.0.0.0/24 njord

docker rm -f njord1 || true
docker rm -f njord2 || true
docker rm -f njord3 || true
docker rm -f njord4 || true
docker rm -f njord5 || true

if [[ "$#" == 1  ]]; then
    docker run -d --rm --name njord1 --network njord --ip 10.0.0.50 -v $(pwd):/app debian:10-slim /app/target/debug/njord -vvvv start --config /app/assets/node1.yaml

    docker run -d --rm --name njord2 --network njord --ip 10.0.0.51 -v $(pwd):/app debian:10-slim /app/target/debug/njord -vvvv start --config /app/assets/node2.yaml

    docker run -d --rm --name njord3 --network njord --ip 10.0.0.52 -v $(pwd):/app debian:10-slim /app/target/debug/njord -vvvv start --config /app/assets/node3.yaml

    docker run -d --rm --name njord4 --network njord --ip 10.0.0.53 -v $(pwd):/app debian:10-slim /app/target/debug/njord -vvvv start --config /app/assets/node4.yaml

    docker run -d --rm --name njord5 --network njord --ip 10.0.0.54 -v $(pwd):/app debian:10-slim /app/target/debug/njord -vvvv start --config /app/assets/node5.yaml

fi
