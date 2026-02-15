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
git clone <URL>
cd web_api
```

### 2. Install dependencies
```
cargo build
```

### 3. Create a .env file from the .env.example template and fill it in
Example content:
```
JWT_SECRET=your_secret_key
JWT_LIFETIME_MINUTES=60
```

### 4. Run the server
```
cargo run
```

The server will be available at http://127.0.0.1:3000

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
