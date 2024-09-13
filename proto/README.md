``` 
protoc --go_out=. --go-grpc_out=. helloworld.proto

protoc --dart_out=grpc:dart_grpc/lib -I. proto/helloworld.proto
```