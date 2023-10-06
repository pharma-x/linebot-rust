# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:1.72.1

ENV APP_ROOT /myapp
WORKDIR $APP_ROOT

# Copy local code to the container image.
COPY . $APP_ROOT

# Install production dependencies and build a release artifact.
RUN apt-get install -y ca-certificates \
    cargo install sqlx-cli --no-default-features --features mysql; \
    cargo build --release;

EXPOSE 3000

# Run the web service on container startup.
ENTRYPOINT ["target/release/one-app"]
