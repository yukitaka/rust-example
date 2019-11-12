# Build Stage
FROM rust:1.39.0 as cargo-build

RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.toml

RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN echo "fn main() {}" > build.rs
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/rust-example*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# Final Stage
FROM alpine:latest

RUN addgroup -g 1000 rust
RUN adduser -D -s /bin/sh -u 1000 -G rust rust
WORKDIR /home/rust/bin/

COPY --from=cargo-build /usr/src/app/target/x86_64-unknown-linux-musl/release/rust-example .

RUN chown rust:rust rust-example

# Cloud Run Defult Port
ENV PORT 8080
EXPOSE 8080

# Run the web service on container startup.
USER rust
CMD ["./rust-example"]
