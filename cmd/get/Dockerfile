FROM rust:buster as builder

WORKDIR /build

RUN apt update -y
RUN apt install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --target x86_64-unknown-linux-musl --release


FROM scratch

COPY --from=builder /etc/ssl/certs /etc/ssl/certs
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/stock /stock

ENTRYPOINT ["/stock"]
