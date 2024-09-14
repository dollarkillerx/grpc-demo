use proto::helloworld::greeter_client::GreeterClient;
use proto::helloworld::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Leo".into(),
    });

    let response = client.say_hello(request).await?;

    println!("Response from server: {}", response.into_inner().message);

    Ok(())
}
