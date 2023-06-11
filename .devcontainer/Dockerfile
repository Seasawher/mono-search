FROM rust:bullseye

WORKDIR /workspace

COPY .. .

# clippy: Rust linter
RUN rustup component add clippy

# clap: Command Line Argument Parser for Rust
RUN cargo add clap --features derive