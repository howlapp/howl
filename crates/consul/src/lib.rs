//! Utility crate for interacting with the Consul agent.

use std::{collections::HashMap, env};

use anyhow::{Context, Result};
use rs_consul::RegisterEntityService;

pub struct ConsulClient {
    client: rs_consul::Consul,
}

impl ConsulClient {
    /// Create a new ConsulClient from the environment variables.
    pub fn from_env() -> Result<Self> {
        if cfg!(debug_assertions) {
            dotenv::dotenv().context("failed to load environment")?;
        }
        let config = rs_consul::Config::from_env();
        let client = rs_consul::Consul::new(config);
        Ok(ConsulClient { client })
    }

    /// List all service kinds registered with Consul.
    pub async fn list_service_kinds(&self) -> Result<Vec<String>> {
        let services = self.client.get_all_registered_service_names(None).await?;
        Ok(services.response)
    }

    /// List all service instances registered with Consul for a given service kind.
    pub async fn list_service_instances(&self, service_kind: &str) -> Result<Vec<(String, u16)>> {
        Ok(self
            .client
            .get_service_addresses_and_ports(service_kind, None)
            .await?)
    }

    /// Register this service instance with Consul.
    pub async fn register(&self, service_kind: &str, port: u16) -> Result<()> {
        let service = if self
            .list_service_kinds()
            .await
            .context("failed to list existing services")?
            .contains(&service_kind.to_string())
        {
            Some(RegisterEntityService {
                Service: service_kind.to_owned(),
                ID: None,
                Tags: vec![],
                TaggedAddresses: HashMap::new(),
                Meta: HashMap::new(),
                Port: Some(port),
                Namespace: None,
            })
        } else {
            None
        };

        self.client
            .register_entity(&rs_consul::RegisterEntityPayload {
                ID: None,
                Node: "".to_string(),
                Address: env::var("CONSUL_ADDR").expect("CONSUL_ADDR not set"),
                Datacenter: None,
                TaggedAddresses: HashMap::new(),
                NodeMeta: HashMap::new(),
                Service: service,
                Check: None,
                SkipNodeUpdate: None,
            })
            .await?;
        Ok(())
    }
}

pub async fn register(service_kind: &str, port: u16) -> Result<()> {
    let client = ConsulClient::from_env()?;
    client
        .register(service_kind, port)
        .await
        .context("failed to register service")?;
    Ok(())
}
