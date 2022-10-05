#!/bin/bash

set -e

cargo run &
cd ../CPServ
cargo run --bin client &
cargo run --bin client &
