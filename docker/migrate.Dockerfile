FROM rust:1.75
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres
COPY tauri/migrations migrations
