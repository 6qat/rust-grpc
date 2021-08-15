use tonic::Response;

use helloworld::greeter_client::GreeterClient;
use helloworld::HelloReply;
use helloworld::HelloRequest;

pub mod helloworld;
// pub mod generated/dtc;
//pub mod helloworld {
//    tonic::include_proto!("helloworld");
//}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("https://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Mr. Tonic".into(),
    });
    let response: Response<HelloReply> = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
