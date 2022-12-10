FROM rust:1.65 as builder

# Install dependency packages
RUN apt update && apt upgrade -y && apt install -y protobuf-compiler libprotobuf-dev

# Copy the current working directory from the host
COPY . /app

# Set the working directory to the app directory
WORKDIR /app

# Build the cargo
RUN cargo build

# Execute app
# FROM gcr.io/distroless/cc-debian11

# COPY --from=builder /app/target/release/ app/*
# WORKDIR /app

# CMD ["cargo", "run"]