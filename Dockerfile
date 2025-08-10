# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files first to leverage Docker's caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy src directory and run `cargo build` to cache dependencies
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Remove the dummy src directory
RUN rm -rf src

# Copy the actual project files into the container
COPY . .

# Build the project
RUN cargo build --release


# Run the application
# CMD ["cargo", "run", "--release"]
