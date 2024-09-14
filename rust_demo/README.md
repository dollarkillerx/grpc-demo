``` 
grpc_demo/
├── proto/         # 维护 proto 文件，并生成共享的 Rust 代码
│   ├── build.rs
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── server/        # gRPC 服务器项目
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── client/        # gRPC 客户端项目
    ├── Cargo.toml
    └── src/
        └── main.rs
        
        
cargo build
```