# AXIOM HIVE - Multi-Stage Docker Build
FROM rust:1.75-slim as rust-builder
WORKDIR /build
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml ./
COPY sap4d ./sap4d
COPY audit ./audit
COPY portal ./portal
RUN cargo build --release --workspace
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=rust-builder /build/target/release/axiom-portal /usr/local/bin/
USER 1000
CMD ["/usr/local/bin/axiom-portal"]
