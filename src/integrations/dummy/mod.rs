use crate::homectl_core::{
    device::Device,
    events::TxEventChannel,
    integration::{Integration, IntegrationId},
};
use async_trait::async_trait;
use serde::Deserialize;
use std::{collections::HashMap, error::Error};

#[derive(Debug, Deserialize)]
pub struct DummyConfig {
    asd: String,
}

pub struct Dummy {
    id: String,
    devices: Vec<Device>,
    sender: TxEventChannel,
    config: DummyConfig,
}

#[async_trait]
impl Integration for Dummy {
    fn new(id: &IntegrationId, config: &config::Value, sender: TxEventChannel) -> Self {
        Dummy {
            id: id.clone(),
            devices: Vec::new(),
            config: config.clone().try_into().unwrap(),
            sender,
        }
    }

    async fn register(&mut self) -> Result<(), Box<dyn Error>> {
        let resp: HashMap<String, String> =
            reqwest::get("https://httpbin.org/ip").await?.json().await?;
        println!("{:#?}", resp);
        println!("registered dummy integration");

        Ok(())
    }

    async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        println!("started dummy integration");

        Ok(())
    }
}
