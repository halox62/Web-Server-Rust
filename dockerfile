FROM rust:1.78 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY plugins ./plugins
COPY frontend ./frontend
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/web_server .
COPY --from=builder /app/plugins ./plugins
COPY --from=builder /app/frontend ./frontend
EXPOSE 8080 9001
CMD ["./web_server"]