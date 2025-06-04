# diesel_demo

## Description

See [diesel documentation here](https://diesel.rs/guides/getting-started).

## Getting Started

### Prerequisites

- Rust
- Docker
- Diesel CLI (`cargo install diesel_cli`)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/liamwh/diesel_demo/
   cd diesel_demo
   ```

2. Set up the database:

   ```bash
   docker compose up -d
   ```

3. Run database migrations:

   ```bash
   diesel migration run
   ```

4. Build the project:

   ```bash
   cargo build
   ```

### Running the application

```bash
cargo run
```
