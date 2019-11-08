# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:1.39.0

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo install

# Run the web service on container startup.
CMD ["rust-example"]
