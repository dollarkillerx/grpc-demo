use tonic::{transport::Server, Request, Response, Status};

pub mod helloworld {
    tonic::include_proto!("helloworld"); // 生成的模块，使用 include_proto! 宏
}

use helloworld::greeter_server::{Greeter, GreeterServer};
use helloworld::{HelloRequest, HelloReply};

// 定义 Greeter 服务的具体实现
#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // 接收 gRPC 请求
    ) -> Result<Response<HelloReply>, Status> {
        let reply = helloworld::HelloReply {
            message: format!("Hello, {}!", request.into_inner().name), // 构造返回消息
        };
        Ok(Response::new(reply)) // 返回 gRPC 响应
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
