FROM rust:1-slim-bullseye as rust-builder

WORKDIR /opt/rust/wof
RUN apt-get update \
    && apt-get install -y --no-install-recommends git pkg-config make
COPY Cargo.toml .
RUN cargo fetch
COPY src src
RUN cargo build --release --features cli

FROM python:3-bullseye as python-builder
COPY --from=rust-builder /opt/rust/wof/target/release/wof /bin/
RUN wof install export

FROM debian:bullseye
RUN apt-get update \
    && apt-get install -y --no-install-recommends python3 ca-certificates \
    && mkdir /root/.wof \
    && ln -s /usr/bin/python3 /usr/local/bin/python
COPY --from=rust-builder /opt/rust/wof/target/release/wof /bin/
COPY --from=python-builder /root/.wof /root/.wof