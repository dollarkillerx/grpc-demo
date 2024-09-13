use tonic::transport::Channel;
use tonic::Request;

pub mod helloworld {
    tonic::include_proto!("helloworld"); // 生成的模块，使用 include_proto! 宏
}

use helloworld::greeter_client::GreeterClient;
use helloworld::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接到 gRPC 服务器
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // 构造请求
    let request = Request::new(HelloRequest {
        name: "Rust".into(),
    });

    // 发送请求并接收响应
    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
