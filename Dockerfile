FROM rust:1.81-bookworm AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.81-bookworm AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.81-bookworm AS builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
RUN cargo build --release --bin ao3pub-back

FROM debian:bookworm-slim AS runner
LABEL maintainer="Nicola Clark <nicola@slottedspoon.dev>"
EXPOSE 8080
WORKDIR /app
COPY --from=builder /app/target/release/ao3pub-back ao3pub-back
ENV BACKEND_LOG_LEVEL=info
ENTRYPOINT ["/app/ao3pub-back"]
RUN apt-get update && apt-get install -y ca-certificates openssl && rm -rf /var/lib/apt/lists/*
