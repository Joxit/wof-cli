FROM rust:1-slim-bookworm as rust-builder

WORKDIR /opt/rust/wof
RUN apt-get update \
    && apt-get install -y --no-install-recommends git pkg-config make
COPY Cargo.toml .
RUN cargo fetch
COPY src src
RUN cargo build --release --features cli

FROM debian:bookworm
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && mkdir /root/.wof
COPY --from=rust-builder /opt/rust/wof/target/release/wof /bin/