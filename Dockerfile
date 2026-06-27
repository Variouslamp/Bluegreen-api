# --- Etapa 1: Compilación ---

FROM rust:1.96 AS builder
WORKDIR /app
COPY /api-code/ .
RUN cargo build --release

# --- Etapa 2: Ejecución ---

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/api-code /app/api-code
RUN chmod +x /app/api-code
CMD ["./api-code"]
