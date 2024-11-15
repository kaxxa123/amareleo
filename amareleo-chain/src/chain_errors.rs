use std::error::Error;
use std::fmt;

/// amareleo-chain custom errors
#[derive(Debug, PartialEq)]
pub enum ChainErrors {
    /// Cannot find Home Dir
    NoHomeDir,

    /// snarkos was not started.
    ProcessNotRunning,

    /// snarkos process was already started.
    /// call end() before starting again.
    ProcessAlreadyStarted,

    /// Cannot get stdout stream for process
    NoStdout,

    /// Cannot find process ready phrase
    CannotFindReady,

    /// Node Configuration Not Found.
    ConfigNodesNotFound,

    /// Node Configuration only supports up to 10 nodes.
    ConfigTooManyNodes,

    /// --dev parameter must NOT be configured
    ConfigRemoveDevArg(String),

    /// Parametes should not contiain whitespace. Split "--network 1" into two parameters.
    ConfigRemoveArgSpace(String),
}

impl fmt::Display for ChainErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ChainErrors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
