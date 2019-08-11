FROM rust:1.31
RUN mkdir -p /usr/src/snake/server
WORKDIR /usr/src/snake/server
RUN rustup install nightly
RUN rustup default nightly
COPY Cargo.toml Cargo.toml
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
# Cause the following cargo build to rebuild only our application
RUN rm -f target/release/deps/snake_server*
COPY . .
RUN cargo build --release
RUN cargo install --path .
CMD ["snake_server"]
