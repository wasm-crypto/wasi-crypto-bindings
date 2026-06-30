mod array_output;
mod options;
pub(crate) use array_output::*;
pub use options::Options;
pub(crate) use options::OptOptions;

pub mod algorithm_type {
    use crate::raw;

    pub trait AlgorithmType {
        const RAW: raw::AlgorithmType;
    }

    #[derive(Debug)]
    pub struct Symmetric;
    #[derive(Debug)]
    pub struct Signatures;
    #[derive(Debug)]
    pub struct KeyExchange;

    impl AlgorithmType for Symmetric {
        const RAW: raw::AlgorithmType = raw::ALGORITHM_TYPE_SYMMETRIC;
    }
    impl AlgorithmType for Signatures {
        const RAW: raw::AlgorithmType = raw::ALGORITHM_TYPE_SIGNATURES;
    }
    impl AlgorithmType for KeyExchange {
        const RAW: raw::AlgorithmType = raw::ALGORITHM_TYPE_KEY_EXCHANGE;
    }
}
