FROM rust:1.75 AS base
RUN apt-get update --yes && \
    apt-get install --yes \
        libsoup2.4-dev \
        javascriptcoregtk-4.0 \
        librust-gdk-sys-dev \
        libatk1.0-dev \
        libwebkit2gtk-4.0-dev
WORKDIR /code
RUN cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo build
COPY src src
COPY icons icons
COPY build.rs tauri.conf.json ./


FROM base AS builder
RUN cargo build --release --offline


FROM debian:buster-slim AS prod
COPY --from=builder /code/target/release/alexandria-backend /
CMD [ "/alexandria-backend" ]
