#!/bin/sh
# $HOME/github.com/loicbourgois/gaime/simple-market/command/bench.sh
set -e
echo "# Start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gaime/simple-market
cargo +nightly fmt
cargo +nightly bench --features bench # 2>&1 | tee $HOME/github.com/loicbourgois/gaime/simple-market/bench.txt
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Duration: $ELAPSED_TIME s"
