FROM rust:bullseye

WORKDIR /workspace

COPY .. .

RUN rustup component add clippy \
    rustfmt
