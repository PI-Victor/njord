#!/usr/bin/env bash
echo "removig container njord1" && docker rm -f njord1 || true
echo "removig container njord2" && docker rm -f njord2 || true
echo "removig container njord3" && docker rm -f njord3 || true
echo "removig container njord4" && docker rm -f njord4 || true
echo "removig container njord5" && docker rm -f njord5 || true

if [[ "$#" == 1  ]]; then
    docker run -u 1000 -w /app -ti -v $(pwd):/app rust:buster cargo build 
    docker network rm njord || true
    docker network create --subnet 10.0.0.0/24 njord

    docker run -d --rm --name njord1 --expose 8718 -p 5000:8717 --network njord --ip 10.0.0.50 -v $(pwd):/opt debian:10-slim /opt/target/debug/njord -vvvv start --config /opt/assets/node1.yaml

    docker run -d --rm --name njord2  --expose 8718 -p 5001:8717 --network njord --ip 10.0.0.51 -v $(pwd):/opt debian:10-slim /opt/target/debug/njord -vvvv start --config /opt/assets/node2.yaml

    docker run -d --rm --name njord3  --expose 8718 -p 5002:8717 --network njord --ip 10.0.0.52 -v $(pwd):/opt debian:10-slim /opt/target/debug/njord -vvvv start --config /opt/assets/node3.yaml

    docker run -d --rm --name njord4  --expose 8718 -p 5003:8717 --network njord --ip 10.0.0.53 -v $(pwd):/opt debian:10-slim /opt/target/debug/njord -vvvv start --config /opt/assets/node4.yaml

    docker run -d --rm --name njord5  --expose 8718 -p 5004:8717 --network njord --ip 10.0.0.54 -v $(pwd):/opt debian:10-slim /opt/target/debug/njord -vvvv start --config /opt/assets/node5.yaml
fi
