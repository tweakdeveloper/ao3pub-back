FROM rust:1.78-slim-bookworm AS builder
WORKDIR /usr/src/ao3pub-back
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim AS runner
LABEL maintainer="Nicola Clark <nicola@slottedspoon.dev>"
EXPOSE 20732
COPY --from=builder /usr/local/cargo/bin/ao3pub-back /usr/local/bin/ao3pub-back
CMD ["ao3pub-back"]
