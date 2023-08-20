
FROM messense/rust-musl-cross:x86_64-musl AS chef
RUN cargo install cargo-chef
WORKDIR /letbe

RUN apt-get update && apt-get install -y protobuf-compiler

FROM chef AS planner
# Copy source code from previous stage
COPY . .

FROM chef AS builder
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /letbe/target/x86_64-unknown-linux-musl/debug/letbe /letbe
ENTRYPOINT ["/letbe"]
EXPOSE 50055