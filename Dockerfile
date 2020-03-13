FROM rust:1-slim-buster as rust-builder

WORKDIR /opt/rust/wof
RUN apt-get update \
    && apt-get install -y --no-install-recommends git pkg-config make
COPY Cargo.toml .
RUN cargo fetch
COPY src src
RUN cargo build --release --features cli

FROM golang:1-buster as go-builder
COPY --from=rust-builder /opt/rust/wof/target/release/wof /bin/
RUN wof install shapefile

FROM python:2-buster as python-builder
COPY --from=rust-builder /opt/rust/wof/target/release/wof /bin/
RUN wof install export

FROM debian:buster
RUN apt-get update \
    && apt-get install -y --no-install-recommends python2 ca-certificates \
    && mkdir /root/.wof \
    && ln -s /usr/bin/python2 /usr/local/bin/python
COPY --from=rust-builder /opt/rust/wof/target/release/wof /bin/
COPY --from=go-builder /root/.wof /root/.wof
COPY --from=python-builder /root/.wof /root/.wof