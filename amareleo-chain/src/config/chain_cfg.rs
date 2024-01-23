use crate::chain_errors::ChainErrors;
use crate::config::*;

use std::fs::File;
use std::fs::{create_dir, remove_dir_all};
use std::io::Read;
use std::path::PathBuf;

use anyhow::{Context, Ok};
use homedir::get_my_home;

impl ChainArgs {
    pub fn init(json_path: &PathBuf) -> anyhow::Result<()> {
        let cfg = ChainArgs {
            snarkos: vec![
                NodeArgs {
                    node: default_node_args(0, false),
                    started: String::from(NODE0_START_COMPLETE),
                },
                NodeArgs {
                    node: default_node_args(1, false),
                    started: String::from(NODE1_START_COMPLETE),
                },
                NodeArgs {
                    node: default_node_args(2, false),
                    started: String::from(NODE2_START_COMPLETE),
                },
                NodeArgs {
                    node: default_node_args(3, false),
                    started: String::from(NODE3_START_COMPLETE),
                },
            ],
        };

        // Open the file for writing
        let json_file = File::create(json_path)?;

        // Serialize the data to the file
        serde_json::to_writer(json_file, &cfg)?;

        Ok(())
    }

    pub fn load(path: &Option<String>) -> anyhow::Result<ChainArgs> {
        let cfg_path = match path {
            Some(apath) => file_exists(apath)?,
            None => create_amareleo_dir()?.join(AMARELEO_CHAIN_CFG),
        };

        if !cfg_path.exists() {
            ChainArgs::init(&cfg_path)?;
        }

        let mut cfg_file = File::open(cfg_path)?;
        let mut raw_cfg = String::new();

        cfg_file.read_to_string(&mut raw_cfg)?;
        let mut json: ChainArgs = serde_json::from_str(&raw_cfg)?;

        // Make sure we have at least one node configured
        if json.snarkos.is_empty() {
            return Err(ChainErrors::ConfigNodesNotFound.into());
        }

        // Make sure we don't have more than 10 nodes configured
        if json.snarkos.len() > 10 {
            return Err(ChainErrors::ConfigTooManyNodes.into());
        }

        // Make sure none of the configurations includes the --dev argument
        for cfgnode in json.snarkos.iter() {
            for param in cfgnode.node.iter() {
                if param.to_ascii_lowercase().trim() == "--dev" {
                    return Err(ChainErrors::ConfigRemoveDevArg.into());
                }
            }
        }

        //Insert the --dev parameter
        for (idx, cfgnode) in json.snarkos.iter_mut().enumerate() {
            cfgnode.node.push(String::from("--dev"));
            cfgnode.node.push(idx.to_string());
        }

        Ok(json)
    }
}

pub fn default_node_args(num: usize, all: bool) -> Vec<String> {
    let mut args: Vec<String> = vec![
        String::from("start"),
        String::from("--nodisplay"),
        String::from("--validator"),
    ];

    if num != 0 {
        args.push(String::from("--norest"));
    }

    if all {
        args.push(String::from("--dev"));
        args.push(num.to_string());
    }

    args
}

pub fn folder_exists(path: &str) -> anyhow::Result<PathBuf> {
    let folder = PathBuf::from(path);

    if !folder.exists() || !folder.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Folder path not found",
        ))
        .context("Finding ledger base directory.");
    }

    Ok(folder)
}

pub fn file_exists(path: &str) -> anyhow::Result<PathBuf> {
    let file = PathBuf::from(path);

    if !file.exists() || !file.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File path not found",
        ))
        .context("Finding configuration file.");
    }

    Ok(file)
}

pub fn create_amareleo_dir() -> anyhow::Result<PathBuf> {
    //Cross-platform compatible retreival of the user's home dir
    let home_path: PathBuf = match get_my_home()?.take() {
        None => return Err(ChainErrors::NoHomeDir.into()),
        Some(path) => path,
    };

    if !home_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home path not found",
        ))
        .context("Failed to create ledger directory");
    }

    if !home_path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Home path is not a directory",
        ))
        .context("Failed to create ledger directory");
    }

    let amareleo_path = home_path.join(AMARELEO_HOME_DIR);
    if !amareleo_path.exists() {
        create_dir(&amareleo_path)?;
    }

    Ok(amareleo_path)
}

pub fn create_ledger_dir() -> anyhow::Result<PathBuf> {
    let amareleo_path = create_amareleo_dir()?;

    let chain_path = amareleo_path.join(AMARELEO_CHAIN_DIR);
    if chain_path.exists() {
        remove_dir_all(&chain_path)?;
    }
    create_dir(&chain_path)?;

    Ok(chain_path)
}

pub fn clear_ledger_dir(path: &PathBuf) -> anyhow::Result<()> {
    if !path.exists() || !path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Chain Path not found",
        ))
        .context("Failed to clear ledger.");
    }

    remove_dir_all(path)?;
    Ok(())
}
