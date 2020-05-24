FROM rust:latest as build

WORKDIR .

RUN GRPC_HEALTH_PROBE_VERSION=v0.3.1 && \
    wget -qO /bin/grpc_health_probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64

RUN apt-get update && \
    apt-get install -y clang && \
    ln -s /usr/bin/g++ /usr/bin/musl-g++

COPY server/build.rs /app/server/
COPY server/proto /app/server/
COPY server/vendor /app/server/
COPY server/.cargo /app/server/
COPY server/src /app/server/
COPY server/Cargo.lock /app/server/
COPY server/Cargo.toml /app/server/

WORKDIR /app/server

RUN rustup component add rustfmt && \
    cargo fetch

RUN cargo build --release

FROM rust:latest

COPY --from=build /app/server/target/release/spq_server /app/spq_server
COPY --from=build /bin/grpc_health_probe /bin/grpc_health_probe
RUN chmod +x /bin/grpc_health_probe

EXPOSE 9090

CMD ["/app/spq_server"]
