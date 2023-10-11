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