FROM rust:1.77
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres
COPY shared/models/migrations migrations
