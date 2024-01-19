use std::collections::HashMap;
use std::sync::Arc;
use log::debug;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use crate::binding::sila_service::{GetFeatureDefinitionParameters, GetFeatureDefinitionResponses, GetImplementedFeaturesParameters, GetImplementedFeaturesResponses, GetServerDescriptionParameters, GetServerDescriptionResponses, GetServerNameParameters, GetServerNameResponses, GetServerTypeParameters, GetServerTypeResponses, GetServerUuidParameters, GetServerUuidResponses, GetServerVendorUrlParameters, GetServerVendorUrlResponses, GetServerVersionParameters, GetServerVersionResponses, SetServerNameParameters, SetServerNameResponses};
use crate::binding::sila_service::si_la_service_server::SiLaService;
use crate::binding::SiLAString;
use crate::sila_domain::server_info::ServerInfo;

static SILA_SERVICE_DEFINITION: &str = include_str!("../../feature_definitions/SiLAService-v1_0.sila.xml");
static SILA_SERVICE_FQN: &str = "org.silastandard/core/SiLAService/v1";

static GREETING_PROVIDER_DEFINITION: &str = include_str!("../../feature_definitions/GreetingProvider-v1_0.sila.xml");
static GREETING_PROVIDER_FQN: &str =  "org.silastandard/examples/GreetingProvider/v1";

pub struct SiLAServerBuilder {
    info: ServerInfo,
    registry: HashMap<String, String>
}

impl SiLAServerBuilder {

    fn with_feature(mut self, fqn: String, definition: String) -> Self {
        self.registry.insert(fqn, definition);
        self
    }

    pub fn new(info: ServerInfo) -> Self {
        SiLAServerBuilder{
            info,
            registry: HashMap::new()
        }
    }

    pub fn with_greeting_provider(mut self) -> Self {
        self.registry.insert(GREETING_PROVIDER_FQN.to_string(), GREETING_PROVIDER_DEFINITION.to_string());
        self
    }

    pub fn build(mut self) -> SiLAServer {
        self.registry.insert(SILA_SERVICE_FQN.to_string(), SILA_SERVICE_DEFINITION.to_string());
        SiLAServer::new(self.info, self.registry)
    }
}

pub struct SiLAServer {
    info: Arc<Mutex<ServerInfo>>,
    registry: HashMap<String, String>,
}

impl SiLAServer {
    fn new(info: ServerInfo, registry: HashMap<String, String>) -> Self {
        SiLAServer {
            info: Arc::new(Mutex::new(info)),
            registry,
        }
    }
}

#[tonic::async_trait]
impl SiLaService for SiLAServer {
    async fn get_feature_definition(&self, request: Request<GetFeatureDefinitionParameters>) -> Result<Response<GetFeatureDefinitionResponses>, Status> {
        let identifier = request.into_inner().feature_identifier;
        log::debug!("requested identifier: {:?}", identifier);

        let definition: Option<SiLAString> = identifier
            .and_then(|x| {
                let temp: String = x.clone().into();
                self.registry.get(&temp)
            }
            ).map(|x| x.to_string().into());

        log::debug!("{:?}", definition);


        Ok(
            Response::new(
                GetFeatureDefinitionResponses {
                    feature_definition: definition,
                }
            ))
    }

    async fn set_server_name(&self, request: Request<SetServerNameParameters>) -> Result<Response<SetServerNameResponses>, Status> {
        let new_name = request.into_inner().server_name;
        if let Some(name) = new_name {
            let mut info = self.info.lock().await;
            info.name = name.into();
        }

        Ok(Response::new(SetServerNameResponses {})
        )
    }

    async fn get_server_name(&self, request: Request<GetServerNameParameters>) -> Result<Response<GetServerNameResponses>, Status> {
        let info = self.info.lock().await;

        Ok(Response::new(
            GetServerNameResponses {
                server_name: Some(info.name.clone().into()),
            }
        ))
    }

    async fn get_server_type(&self, request: Request<GetServerTypeParameters>) -> Result<Response<GetServerTypeResponses>, Status> {
        let info = self.info.lock().await;
        Ok(
            Response::new(
                GetServerTypeResponses {
                    server_type: Some(info.server_type.clone().into()),
                }
            )
        )
    }

    async fn get_server_uuid(&self, request: Request<GetServerUuidParameters>) -> Result<Response<GetServerUuidResponses>, Status> {
        let info = self.info.lock().await;
        Ok(
            Response::new(
                GetServerUuidResponses {
                    server_uuid: Some(info.uuid.to_string().into()),
                }
            )
        )
    }

    async fn get_server_description(&self, request: Request<GetServerDescriptionParameters>) -> Result<Response<GetServerDescriptionResponses>, Status> {
        let info = self.info.lock().await;
        Ok(
            Response::new(
                GetServerDescriptionResponses {
                    server_description: Some(info.description.to_string().into()),
                }
            )
        )
    }

    async fn get_server_version(&self, request: Request<GetServerVersionParameters>) -> Result<Response<GetServerVersionResponses>, Status> {
        let info = self.info.lock().await;
        Ok(
            Response::new(
                GetServerVersionResponses {
                    server_version: Some(info.version.clone().into()),
                }
            )
        )
    }

    async fn get_server_vendor_url(&self, request: Request<GetServerVendorUrlParameters>) -> Result<Response<GetServerVendorUrlResponses>, Status> {
        let info = self.info.lock().await;
        Ok(
            Response::new(
                GetServerVendorUrlResponses {
                    server_vendor_url: Some(info.vendor_url.to_string().clone().into()),
                }
            )
        )
    }

    async fn get_implemented_features(&self, request: Request<GetImplementedFeaturesParameters>) -> Result<Response<GetImplementedFeaturesResponses>, Status> {
        Ok(
            Response::new(
                GetImplementedFeaturesResponses {
                    implemented_features: self.registry.keys().map(|x| {
                        let temp: crate::binding::sila_standard::String = x.to_string().into();
                        temp
                    }
                    ).collect()
                }
            )
        )
    }
}