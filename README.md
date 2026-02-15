# web_api

Web API in Rust using Axum and JWT authentication

## Description
This project implements a simple web server in Rust using the [Axum](https://github.com/tokio-rs/axum) framework. JWT (JSON Web Token) is used for authentication. The server supports basic routes for authorization and token validation.

## Features
- Authorization via client_id and client_secret
- JWT token generation
- Token validation on a protected route
- Global config with .env file support
- Asynchronous request handling

## Quick Start

### 1. Clone the repository and navigate to the project folder
```
git clone https://github.com/Alex-10111/jwt_provider.git
cd jwt_provider
```

### 2. Install dependencies
```
cargo build
```

### 3. Create a .env file from the .env.example template and fill it in

### 4. Run the server
```
cargo run
```

The server will be available at http://127.0.0.1:8001 (by default, can be changed in .env).

## Example Requests

### Get a token
```
POST /authorize
Content-Type: application/json

{
  "client_id": "foo",
  "client_secret": "bar"
}
```
Response:
```
{
  "access_token": "<JWT>",
  "token_type": "Bearer"
}
```

### Validate a token
```
GET /check
Authorization: Bearer <JWT>
```
Response:
```
You were successfully authenticated! :)
Your data:
Email: ...
Company: ...
```

## Project Structure
- `src/main.rs` — entry point, server startup
- `src/routes.rs` — route handlers
- `src/structs.rs` — data structures, JWT handling, errors
- `src/config.rs` — global config and .env support
- `src/utils.rs` — utility functions

## Dependencies
- [axum](https://crates.io/crates/axum)
- [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
- [dotenv](https://crates.io/crates/dotenv)
- [serde](https://crates.io/crates/serde)
- [tokio](https://crates.io/crates/tokio)
- [tracing](https://crates.io/crates/tracing)

## License
MIT
