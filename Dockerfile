FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release
RUN mv ./target/release/rentman-harvest ./app

FROM gcr.io/distroless/cc AS runtime
WORKDIR /app
COPY --from=builder /app/app /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/app"]

# FROM lukemathwalker/cargo-chef:latest AS chef
# WORKDIR /app

# FROM chef AS planner
# COPY ./Cargo.toml ./Cargo.lock ./
# COPY ./src ./src
# RUN cargo chef prepare

# FROM chef AS builder
# COPY --from=planner /app/recipe.json .
# RUN cargo chef cook --release
# COPY . .
# RUN cargo build --release
# RUN mv ./target/release/rentman-harvest ./app

# FROM debian:bookworm-slim AS runtime
# WORKDIR /app
# COPY --from=builder /app/app /usr/local/bin/
# ENTRYPOINT ["/usr/local/bin/app"]