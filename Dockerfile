FROM rust:1.82 AS builder

# Create app directory
WORKDIR /usr/src/nordiv

# Copy Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a minimal base image for the final stage
FROM gcr.io/distroless/cc

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/nordiv/target/release/nordiv /usr/local/bin/nordiv

# Set the binary as the entrypoint
ENTRYPOINT ["/usr/local/bin/nordiv"]
