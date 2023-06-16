#!/bin/sh
# $HOME/github.com/loicbourgois/gaime/simple-market/command/run.sh
set -e
echo "# Start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gaime/simple-market
rustup override set stable
cargo +nightly fmt
RUST_BACKTRACE=1 cargo test
cargo clippy -- \
    -A clippy::single_match \
    -A clippy::too_many_arguments \
    -W clippy::pedantic \
    -A clippy::cast_precision_loss \
    -A clippy::cast_sign_loss \
    -A clippy::cast_possible_truncation \
    -A clippy::module_name_repetitions \
    -A clippy::unused_self \
    -A clippy::match_same_arms \
    -A clippy::similar_names \
    -A clippy::many_single_char_names \
    -A clippy::match_on_vec_items \
    -A clippy::single_match_else \
    -A clippy::vec_init_then_push \
    -A clippy::missing_errors_doc \
    -A clippy::missing_panics_doc \
    -A clippy::too_many_lines
cargo +nightly fmt
RUST_BACKTRACE=1 cargo run --release
# docker run --rm -u `id -u`:`id -g` -v $HOME/github.com/loicbourgois/gaime/simple-market/latest/:/data minlag/mermaid-cli -i 0_nn_small.mmd -o 0_nn_small.png -w 5000 -H 5000
# docker run --rm -u `id -u`:`id -g` -v $HOME/github.com/loicbourgois/gaime/simple-market/latest/:/data minlag/mermaid-cli -i 0_nn.mmd -o 0_nn.png -w 5000 -H 5000
# docker run --rm -u `id -u`:`id -g` -v $HOME/github.com/loicbourgois/gaime/simple-market/latest/:/data minlag/mermaid-cli -i 1_nn_small.mmd -o 1_nn_small.png -w 5000 -H 5000
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Duration: $ELAPSED_TIME s"
