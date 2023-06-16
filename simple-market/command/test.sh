#!/bin/sh
# $HOME/github.com/loicbourgois/gaime/simple-market/command/test.sh
set -e
echo "# Start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gaime/simple-market
cargo +nightly fmt
RUST_BACKTRACE=full cargo +nightly test --features test # 2>&1 | tee $HOME/github.com/loicbourgois/gaime/simple-market/bench.txt
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Duration: $ELAPSED_TIME s"
