use tonic::{Request, Response, Status};
use tonic::transport::Server;
use url::Url;
use uuid::Uuid;
use siloxide::binding::sila_greeting_provider::greeting_provider_server::{GreetingProvider, GreetingProviderServer};
use siloxide::binding::sila_greeting_provider::{GetStartYearParameters, GetStartYearResponses, SayHelloParameters, SayHelloResponses};
use siloxide::binding::sila_service::si_la_service_server::SiLaServiceServer;
use siloxide::server::SiLAServerBuilder;
use siloxide::sila_domain::error::SiLAError;
use siloxide::sila_domain::server_info::ServerInfo;

pub struct GreetingProviderCarrier {}

#[tonic::async_trait]
impl GreetingProvider for GreetingProviderCarrier {
    async fn say_hello(&self, request: Request<SayHelloParameters>) -> Result<Response<SayHelloResponses>, Status> {
        let param = request.into_inner().name;

        if let Some(name) = &param {
            log::debug!("received name {:?}", param);
            return Ok(
                Response::new(
                    SayHelloResponses {
                        greeting: Some(format!("Hello {:?}, this is Rust calling", param).into()),
                    }
                )
            );
        } else {
            let err = SiLAError::ValidationError {
                parameter_fqi: "".to_string(),
                content: "you forgot the parameter, you full".to_string(),
            };


            return Err(
                err.into()
            );
        }
    }

    async fn get_start_year(&self, request: Request<GetStartYearParameters>) -> Result<Response<GetStartYearResponses>, Status> {
        Ok(
            Response::new(
                GetStartYearResponses {
                    start_year: Some(2024.into()),
                }
            )
        )
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let info = ServerInfo {
        name: "SiLA Basic Server".to_string(),
        server_type: "Device".to_string(),
        uuid: Uuid::parse_str("4daafeef-540e-4508-81e1-ea3a5232fb52").unwrap(),
        vendor_url: Url::parse("https://www.smartlab-solutions.de").unwrap(),
        description: "This is a test server for SiLA".to_string(),
        version: "0.1".to_string(),
    };


    let addr = "192.168.1.175:50052".parse().unwrap();
    let sila_server = SiLAServerBuilder::new(info)
        .with_greeting_provider()
        .build();


    Server::builder().add_service(SiLaServiceServer::new(sila_server))
        .add_service(GreetingProviderServer::new(GreetingProviderCarrier {}))
        .serve(addr)
        .await?;

    Ok(())
}