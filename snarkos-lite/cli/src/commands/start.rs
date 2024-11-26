// Copyright 2024 Aleo Network Foundation
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::Parser;
use anyhow::{Ok, Result};

use core::str::FromStr;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json;

use std::{
    net::SocketAddr,
    path::PathBuf
};

/// A mapping of `staker_address` to `(validator_address, withdrawal_address, amount)`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BondedBalances(IndexMap<String, (String, String, u64)>);

impl FromStr for BondedBalances {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

/// Starts the snarkOS-lite node.
#[derive(Clone, Debug, Parser)]
pub struct Start {
    /// Specify the network ID of this node
    #[clap(default_value = "0", long = "network")]
    pub network: u16,

    /// Specify this node as a validator
    #[clap(long = "validator")]
    pub validator: bool,
    /// Specify this node as a prover
    #[clap(long = "prover")]
    pub prover: bool,
    /// Specify this node as a client
    #[clap(long = "client")]
    pub client: bool,

    /// Specify the account private key of the node
    #[clap(long = "private-key")]
    pub private_key: Option<String>,
    /// Specify the path to a file containing the account private key of the node
    #[clap(long = "private-key-file")]
    pub private_key_file: Option<PathBuf>,

    /// Specify the IP address and port for the node server
    #[clap(long = "node")]
    pub node: Option<SocketAddr>,
    /// Specify the IP address and port for the BFT
    #[clap(long = "bft")]
    pub bft: Option<SocketAddr>,
    /// Specify the IP address and port of the peer(s) to connect to
    #[clap(default_value = "", long = "peers")]
    pub peers: String,
    /// Specify the IP address and port of the validator(s) to connect to
    #[clap(default_value = "", long = "validators")]
    pub validators: String,
    /// If the flag is set, a node will allow untrusted peers to connect
    #[clap(long = "allow-external-peers")]
    pub allow_external_peers: bool,

    /// Specify the IP address and port for the REST server
    #[clap(long = "rest")]
    pub rest: Option<SocketAddr>,
    /// Specify the requests per second (RPS) rate limit per IP for the REST server
    #[clap(default_value = "10", long = "rest-rps")]
    pub rest_rps: u32,
    /// If the flag is set, the node will not initialize the REST server
    #[clap(long)]
    pub norest: bool,

    /// If the flag is set, the node will not render the display
    #[clap(long)]
    pub nodisplay: bool,
    /// Specify the verbosity of the node [options: 0, 1, 2, 3, 4]
    #[clap(default_value = "1", long = "verbosity")]
    pub verbosity: u8,
    /// Specify the path to the file where logs will be stored
    #[clap(default_value_os_t = std::env::temp_dir().join("snarkos.log"), long = "logfile")]
    pub logfile: PathBuf,
    /// Enables the metrics exporter
    #[clap(default_value = "false", long = "metrics")]
    pub metrics: bool,

    /// Specify the path to a directory containing the storage database for the ledger
    #[clap(long = "storage")]
    pub storage: Option<PathBuf>,
    /// Enables the node to prefetch initial blocks from a CDN
    #[clap(long = "cdn")]
    pub cdn: Option<String>,
    /// If the flag is set, the node will not prefetch from a CDN
    #[clap(long)]
    pub nocdn: bool,

    /// Enables development mode, specify a unique ID for this node
    #[clap(long)]
    pub dev: Option<u16>,
    /// If development mode is enabled, specify the number of genesis validators (default: 4)
    #[clap(long)]
    pub dev_num_validators: Option<u16>,
    /// If developtment mode is enabled, specify whether node 0 should generate traffic to drive the network
    #[clap(default_value = "false", long = "no-dev-txs")]
    pub no_dev_txs: bool,

    /// If development mode is enabled, specify the custom bonded balances as a JSON object (default: None)
    #[clap(long)]
    pub dev_bonded_balances: Option<BondedBalances>,
}

impl Start {
    /// Starts the snarkOS-lite node.
    pub fn parse(self) -> Result<String> {
        Ok("".to_string())
    }
}