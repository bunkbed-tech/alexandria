FROM rust:1.83
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres
COPY migrations migrations
