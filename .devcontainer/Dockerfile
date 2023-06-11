FROM rust:bullseye

WORKDIR /workspace

COPY .. .

# clippy: Rust linter
RUN rustup component add clippy \
    rustfmt
