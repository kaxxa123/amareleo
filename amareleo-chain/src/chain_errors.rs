use std::error::Error;
use std::fmt;

/// amareleo-chain custom errors
#[derive(Debug, PartialEq)]
pub enum ChainErrors {
    /// Cannot get stdout stream for process
    NoProcessStdout,

    /// Cannot find process ready phrase
    CannotFindReady,
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
