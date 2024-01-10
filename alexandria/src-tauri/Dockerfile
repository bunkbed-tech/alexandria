FROM rust:1.75 AS base

RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres
RUN apt-get update --yes && \
    apt-get install --yes \
        libsoup2.4-dev \
        javascriptcoregtk-4.0 \
        librust-gdk-sys-dev \
        libatk1.0-dev \
        libwebkit2gtk-4.0-dev

WORKDIR /code
EXPOSE 8080

RUN cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo build
COPY . .


FROM base AS builder
RUN cargo build --release --offline


FROM debian:buster-slim AS prod
COPY --from=builder /code/target/release/alexandria-backend /
CMD [ "/alexandria-backend" ]
