#!/bin/bash
cd sakura
cargo build
cd ..

sudo docker-compose build --no-cache
sudo docker-compose up