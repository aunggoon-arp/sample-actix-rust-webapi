# Actix JWT example

This repository provides an example of:

- Actix REST API
- Actix CORS config
- Error handling
- JWT authentication
- Interaction with the database
- Password encryption
- Payload validation

## Required

- Rust
- Docker and docker-compose or Postgresql server

## Usage

- edit .env
- `cargo run --release` or `debug with vscode`

## Api

- POST `/register` - required fields: `name, email, password`, returns bearer token
- POST `/login` - required fields: `email, password`, returns bearer token
- GET `/` - returns home
