mod chain_cfg;

use serde::Deserialize;
use serde::Serialize;

pub use chain_cfg::*;

pub const AMARELEO_HOME_DIR: &str = ".amareleo";
pub const AMARELEO_CHAIN_DIR: &str = "chain";
pub const AMARELEO_CHAIN_CFG: &str = "chain-cfg.json";

pub const NODE0_START_COMPLETE: &str = "No connected validators";
pub const NODE1_START_COMPLETE: &str = "Connected to 1 validators";
pub const NODE2_START_COMPLETE: &str = "Connected to 2 validators";
pub const NODE3_START_COMPLETE: &str = "Advanced to block";

#[derive(Deserialize, Serialize, Debug)]
pub struct NodeArgs {
    pub node: Vec<String>,
    pub started: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChainArgs {
    pub snarkos: Vec<NodeArgs>,
}
