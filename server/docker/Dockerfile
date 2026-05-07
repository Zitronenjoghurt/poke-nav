FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /pokenav
RUN apt-get update && apt-get install -y pkg-config && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /pokenav/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin poke-nav-server

WORKDIR /pokenav/app
RUN trunk build --release

FROM gcr.io/distroless/cc-debian13:nonroot AS runtime
WORKDIR /pokenav
COPY --from=builder /pokenav/target/release/poke-nav-server /usr/local/bin/server
COPY --from=builder /pokenav/app/dist /pokenav/static
CMD ["/usr/local/bin/server"]