# Run the Demo
dev: up
    cargo run --bin diesel_demo

# Start Docker Compose
up:
    docker compose -f compose.yaml up -d
