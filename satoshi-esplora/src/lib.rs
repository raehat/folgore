use clightningrpc_common::json_utils;
use clightningrpc_plugin::errors::PluginError;
use clightningrpc_plugin::types::LogLevel;
use esplora_client::{BlockingClient, Builder, Error};
use satoshi_common::client::SatoshiBackend;
use serde_json::json;

#[derive(Clone)]
enum Network {
    Bitcoin(String),
    Testnet(String),
    Liquid(String),
    BitcoinTor(String),
    TestnetTor(String),
    LiquidTor(String),
}

impl Network {
    pub fn url(&self) -> String {
        match &self {
            Self::Bitcoin(url) => url.to_string(),
            Self::Liquid(url) => url.to_string(),
            Self::Testnet(url) => url.to_string(),
            Self::BitcoinTor(url) => url.to_string(),
            Self::TestnetTor(url) => url.to_string(),
            Self::LiquidTor(url) => url.to_string(),
        }
    }
}

impl TryFrom<&str> for Network {
    type Error = PluginError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "bitcoin" => Ok(Self::Bitcoin("https://blockstream.infoi/api".to_owned())),
            "bitcoin/tor" => Ok(Self::BitcoinTor(
                "http://explorerzydxu5ecjrkwceayqybizmpjjznk5izmitf2modhcusuqlid.onion/api"
                    .to_owned(),
            )),
            "testnet" => Ok(Self::Testnet(
                "https://blockstream.info/testnet/api".to_owned(),
            )),
            "testnet/tor" => Ok(Self::TestnetTor(
                "http://explorerzydxu5ecjrkwceayqybizmpjjznk5izmitf2modhcusuqlid.onion/testnet/api"
                    .to_owned(),
            )),
            _ => Err(PluginError::new(
                -1,
                &format!("network {value} not supported"),
                None,
            )),
        }
    }
}

/// convert the error to a plugin error
fn from(value: Error) -> PluginError {
    PluginError::new(-1, &format!("{value}"), None)
}

#[derive(Clone)]
pub struct Esplora {
    network: Network,
    client: BlockingClient,
}

impl Esplora {
    pub fn new(network: &str) -> Result<Self, PluginError> {
        let network = Network::try_from(network)?;
        let builder = Builder::new(&network.url());
        Ok(Self {
            network,
            client: builder.build_blocking().unwrap(),
        })
    }
}

impl<T: Clone> SatoshiBackend<T> for Esplora {
    fn sync_block_by_height(
        &self,
        _: &mut clightningrpc_plugin::plugin::Plugin<T>,
        height: u64,
    ) -> Result<serde_json::Value, PluginError> {
        todo!()
    }

    fn sync_chain_info(
        &self,
        plugin: &mut clightningrpc_plugin::plugin::Plugin<T>,
    ) -> Result<serde_json::Value, PluginError> {
        let current_height = self.client.get_height().map_err(from)?;
        plugin.log(
            LogLevel::Info,
            &format!("blockchain height: {current_height}"),
        );
        let genesis = self.client.get_blocks(Some(0)).map_err(from)?;

        let genesis = genesis.first().clone().unwrap();
        let network = match genesis.id.to_string().as_str() {
            "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f" => "main",
            "000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943" => "test",
            "1466275836220db2944ca059a3a10ef6fd2ea684b0688d2c379296888a206003" => "liquidv1",
            _ => panic!(""),
        };

        let mut response = json_utils::init_success_response("getinfo".into());
        json_utils::add_str(&mut response, "chain", network);
        json_utils::add_number(&mut response, "headercount", current_height.into());
        json_utils::add_number(&mut response, "blockcount", current_height.into());
        json_utils::add_bool(&mut response, "ibd", false);
        Ok(response)
    }

    fn sync_estimate_fees(
        &self,
        _: &mut clightningrpc_plugin::plugin::Plugin<T>,
    ) -> Result<serde_json::Value, PluginError> {
        todo!()
    }

    fn sync_get_utxo(
        &self,
        _: &mut clightningrpc_plugin::plugin::Plugin<T>,
        _: &str,
        _: u64,
    ) -> Result<serde_json::Value, PluginError> {
        todo!()
    }

    fn sync_send_raw_transaction(
        &self,
        _: &mut clightningrpc_plugin::plugin::Plugin<T>,
        _: &str,
        _: bool,
    ) -> Result<serde_json::Value, PluginError> {
        todo!()
    }
}
