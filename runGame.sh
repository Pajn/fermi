#!/bin/bash

cargo build
RUST_BACKTRACE=1 ./halite -t -d "30 30" "target/debug/my_bot" "target/debug/random_bot"