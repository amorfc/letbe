FROM messense/rust-musl-cross:x86_64-musl as builder
RUN cargo install cargo-chef
WORKDIR /letbe

RUN apt-get update && apt-get install -y protobuf-compiler

#Copy source code
COPY . .

#Build letbelication
RUN cargo build --release --target x86_64-unknown-linux-musl

CMD ["cargo", "run","development", "--release"]
EXPOSE 50055