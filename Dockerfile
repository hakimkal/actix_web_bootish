# Use Rust slim for the builder stage
FROM rust:1.85-slim as planner
WORKDIR /usr/src/app

# Install dependencies for cargo-chef
RUN apt-get update && apt-get install -y \
    pkg-config \
    curl \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-chef
RUN cargo install cargo-chef --version 0.1.71

# Copy only the files needed for planning
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Generate a recipe for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

# Build stage
FROM rust:1.85-slim as builder
WORKDIR /usr/src/app

# Install dependencies for cargo-chef
RUN apt-get update && apt-get install -y \
    pkg-config \
    curl \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-chef
RUN cargo install cargo-chef --version 0.1.71

# Copy the recipe from the planner stage
COPY --from=planner /usr/src/app/recipe.json recipe.json

# Build dependencies (cached layer)
RUN cargo chef cook --release --recipe-path recipe.json

# Copy the rest of the application code
COPY . .

COPY migrations ./migrations

# Build the application with parallel jobs
RUN cargo build --release -j $(nproc)

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app

# Install required runtime dependencies (bash, OpenSSL 3, and certificates)
RUN apt-get update && apt-get install -y \
    bash \
    curl \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/actix_web_boot_app /app/actix_web_boot_app

# Set execute permissions
RUN chmod +x /app/actix_web_boot_app

# Set bash as the default shell
SHELL ["/bin/bash", "-c"]

# Set the entrypoint
CMD ["/app/actix_web_boot_app"]
