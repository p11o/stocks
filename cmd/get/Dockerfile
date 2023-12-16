FROM rust:latest as builder

WORKDIR /app

# Copy the manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Cache the dependencies - this layer is rebuilt only when Cargo.toml or Cargo.lock changes
RUN cargo build --release

# Copy the source and build the application
COPY ./src ./src
RUN cargo build --release

# Stage 2: Create the final image
FROM scratch

# Copy the binary from the builder stage
COPY --from=builder /stock-get/target/release/stock-get /usr/local/bin/stock

# Set the default command to run the binary
CMD ["stock"]
