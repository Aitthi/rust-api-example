# Rust-api-example
a simple example of rust api

## OpenAPI UI
```
http://localhost:8080/apidoc
```

## Folder structure
```
.
├── Cargo.toml # workspace members
├── api
|   ├── som_api
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── lib.rs
│   │   │   ├── routes.rs
│   │   │   ├── api_doc.rs
│   │   │   ├── resource
│   │   │   │   ├── mod.rs
│   │   │   │   └── som_func.rs
├── runtime
│   ├── Cargo.toml
│   ├── src
│   │   └── main.rs
├── package
|   ├── som_package
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── lib.rs
│   │   │   └── som_func.rs
└── 
```

## Development

To develop the application, you will need to install the following tools:

1. Clone the repository:
   ```sh
   git clone https://github.com/Aitthi/rust-api-example.git
   ```
2. Change to the project directory:
   ```sh
   cd rust-api-example
   ```
3. Generate an RSA certificate for JWT:
   ```sh
   openssl genrsa -out ./config/jwt/private.key 4096
   ```
   ```sh
   openssl rsa -in ./config/jwt/private.key -pubout -outform PEM -out ./config/jwt/public.key
   ```
4. Run the application:
   ```sh
   cargo run
   ```
   Or, to automatically rebuild and restart:
   ```sh
   cargo watch -q -c -x 'run'