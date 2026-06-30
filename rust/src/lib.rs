mod asymmetric_common;
mod common;
mod raw;

pub use common::{algorithm_type, Options};

pub mod error;
pub mod kx;
pub mod signatures;
pub mod symmetric;

pub mod prelude {
    pub use crate::{algorithm_type, Options};
    pub use crate::error::Error as WasiCryptoError;
    pub use crate::kx::*;
    pub use crate::signatures::*;
    pub use crate::symmetric::*;
}

#[cfg(test)]
mod test;
