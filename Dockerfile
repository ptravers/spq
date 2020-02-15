FROM rust:latest as build

WORKDIR .

RUN GRPC_HEALTH_PROBE_VERSION=v0.3.1 && \
    wget -qO /bin/grpc_health_probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64

COPY . /app/

WORKDIR /app/server
RUN rustup component add rustfmt --toolchain 1.40.0-x86_64-unknown-linux-gnu && \
    cargo fetch

RUN cargo build --release

FROM rust:latest

COPY --from=build /app/server/target/release/spq_server /app/spq_server
COPY --from=build /bin/grpc_health_probe /bin/grpc_health_probe
RUN chmod +x /bin/grpc_health_probe

EXPOSE 9090

CMD ["/app/spq_server"]
