use clap::{Args, Parser, Subcommand};

const CMD_INIT: &str = "init";
const CMD_RUN: &str = "run";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppCmds {
    #[clap(subcommand)]
    pub cmd_type: Option<AppCmdType>,
}

#[derive(Debug, Subcommand)]
pub enum AppCmdType {
    /// Generate default configuration
    Init(InitArgs),

    /// Run the amareleo-chain
    Run(RunArgs),
}

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Path to json configuration
    #[arg(long, short)]
    pub cfg: Option<String>,

    /// Force writing even if config already exists
    #[arg(long, short)]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct RunArgs {
    /// Path to json configuration
    #[arg(long, short)]
    pub cfg: Option<String>,

    /// Path under which the ledger folders are created.
    #[arg(long, short)]
    pub ledger: Option<String>,
}

pub fn cmd_usage() {
    println!(
        r#"

Examples:

amareleo-chain {CMD_INIT} --cfg ~/mycfg.json

amareleo-chain {CMD_RUN}  --cfg ~/mycfg.json  --ledger  ~/chain
"#
    );
}
