FROM rust:1.85.0 AS builder

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release --bin email_newsletter

FROM rust:1.85.0 AS runtime

COPY --from=builder /app/target/release/email_newsletter email_newsletter

COPY config config

ENV APP_ENVIRONMENT=production

ENTRYPOINT ["./email_newsletter"]
