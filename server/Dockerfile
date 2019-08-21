FROM rust:1.31
RUN mkdir -p /usr/src/server
WORKDIR /usr/src/server
RUN rustup install nightly
RUN rustup default nightly
COPY Cargo.toml Cargo.toml
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
COPY ./src ./src
RUN rm -f target/release/deps/server*
RUN cargo build --release
RUN cargo install --path .
CMD ["server"]
