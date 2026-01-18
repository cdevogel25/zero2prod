# use latest stable rust
# build
FROM lukemathwalker/cargo-chef:latest-rust-slim-trixie AS chef
WORKDIR /app

# this is what you get for developing on two different machines
ARG ARCH_TARGET=x86_64
ENV TARGET=${ARCH_TARGET}

RUN rustup target add ${TARGET}-unknown-linux-musl
RUN apt update && apt install musl-tools musl-dev -y
FROM chef AS planner
COPY . .
# compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# build our project dependencies, not the application!
RUN cargo chef cook --release --recipe-path recipe.json
# up to this point, if our dep tree hasn't changed, all
# layers should be cached.
COPY . .
ENV SQLX_OFFLINE=true
# she static on my link til' I hit the ENTRYPOINT
# build project
RUN cargo build --target=${TARGET}-unknown-linux-musl --release --bin zero2prod

FROM alpine:latest AS runtime
ARG ARCH_TARGET=x86_64
ENV TARGET=${ARCH_TARGET}
WORKDIR /app
RUN apk update \
    && apk add openssl ca-certificates
COPY --from=builder /app/target/${TARGET}-unknown-linux-musl/release/zero2prod ./
COPY configuration configuration
ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./zero2prod"]