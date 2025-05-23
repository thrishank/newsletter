# Rust Server

A Email newsletter server in rust

## Features

- CI Pipeline
  - code testing
  - code formatting
  - code linting
- CD pipeline
  - build the docker image
  - Push the docker images to the registry
  - ssh into the server securely and run the [deploy](https://github.com/thrishank/newsletter/blob/main/scripts/deploy.sh) script
  - Docker Build caching, reduced the build time from 6m 43s to 45s
- API Routes
  - /health_check api to check if the server is running returns nothing takes nothing
  - /subscriptions api to add a new subscription, body: email and name add's into the database
- Tests
  - health_check test to check if the server is running. check if returns 200 OK
  - subscriptions test with valid data checks if the data is added to the database and invalid data checks if the response is 400

## Notes

- Implemented the CI pipeline on github with 3 jobs(cargo test, cargo fmt -- check, cargo clippy -- -D warnings)
- Implemented the /health_check api route returns 200 OK
- Made the port on which server runs dyanmic. This is done by starting a server on port 0. This is telling the OS to pick free port from its ephemeral port range (e.g., 49152–65535 on most systems). This way we can spwan as many server as we want and avoid conflicts in tests
- confif/local.yaml and prod.yaml config in production. The config is loaded using the config crate.

## TODO

- cargo clippy is failing with some warnings. Need to fix them
