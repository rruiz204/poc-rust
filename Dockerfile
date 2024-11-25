FROM rust:alpine3.20 AS builder

WORKDIR /builder

COPY . .

RUN cargo build --release

FROM rust:alpine3.20 AS runtime

WORKDIR /app

COPY --from=builder /builder/target/release .

CMD ["./mainner"]