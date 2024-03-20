FROM rust:1.75-bookworm as dev
WORKDIR /usr/src/app
RUN cargo install cargo-watch
COPY Cargo.toml Cargo.lock ./
RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build && \
    cargo clean -p rust01
COPY . .
CMD ["cargo", "watch", "-x", "run"]

FROM rust:1.75-bookworm as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /usr/src/app/target/release/rust01 /
COPY Rocket.toml /
CMD ["./rust01"]
