# Rust Server

A Email newsletter server in rust

## Features

- CI Pipeline
  - code testing
  - code formatting
  - code linting
- API Routes
  - /health_check api to check if the server is running returns nothing takes nothing

## Notes

- Implemented the CI pipeline on github with 3 jobs(cargo test, cargo fmt -- check, cargo clippy -- -D warnings)
- Implemented the /health_check api route returns 200 OK
- Made the port on which server runs dyanmic. This is done by starting a server on port 0. This is telling the OS to pick free port from its ephemeral port range (e.g., 49152–65535 on most systems). This way we can spwan as many server as we want and avoid conflicts in tests
