mod state;
pub use state::*;

use crate::common::*;
use crate::error::*;
use crate::raw;

pub type SymmetricOptions = Options<algorithm_type::Symmetric>;

struct OptSymmetricKey;

impl OptSymmetricKey {
    fn none() -> raw::OptSymmetricKey {
        raw::OptSymmetricKey {
            tag: raw::OPT_SYMMETRIC_KEY_U_NONE.raw(),
            u: raw::OptSymmetricKeyUnion { none: () },
        }
    }

    fn some(symmetric_key: &SymmetricKey) -> raw::OptSymmetricKey {
        raw::OptSymmetricKey {
            tag: raw::OPT_SYMMETRIC_KEY_U_SOME.raw(),
            u: raw::OptSymmetricKeyUnion {
                some: symmetric_key.handle,
            },
        }
    }
}

#[derive(Debug)]
pub struct SymmetricKey {
    pub(crate) handle: raw::SymmetricKey,
    pub alg: &'static str,
}

impl SymmetricKey {
    pub fn generate(
        alg: &'static str,
        options: Option<&SymmetricOptions>,
    ) -> Result<SymmetricKey, Error> {
        let opt_options = if let Some(options) = options {
            OptOptions::some(options)
        } else {
            OptOptions::none()
        };
        let handle = unsafe { raw::symmetric_key_generate(alg, opt_options) }?;
        Ok(SymmetricKey { handle, alg })
    }

    pub fn from_raw(alg: &'static str, encoded: impl AsRef<[u8]>) -> Result<Self, Error> {
        let encoded = encoded.as_ref();
        let handle = unsafe { raw::symmetric_key_import(alg, encoded.as_ptr(), encoded.len()) }?;
        Ok(SymmetricKey { handle, alg })
    }

    pub fn raw(&self) -> Result<Vec<u8>, Error> {
        let array_handle = unsafe { raw::symmetric_key_export(self.handle) }?;
        ArrayOutput::new(array_handle).into_vec()
    }
}

impl Drop for SymmetricKey {
    fn drop(&mut self) {
        let _ = unsafe { raw::symmetric_key_close(self.handle) };
    }
}

#[derive(Debug)]
pub struct Tag {
    handle: raw::SymmetricTag,
    closed: bool,
}

impl Tag {
    fn new(handle: raw::SymmetricTag) -> Self {
        Tag {
            handle,
            closed: false,
        }
    }

    pub fn into_bytes(mut self) -> Vec<u8> {
        let len = unsafe { raw::symmetric_tag_len(self.handle) }
            .expect("symmetric_tag_len should not fail for valid tag handle");
        let mut bytes = vec![0u8; len];
        unsafe { raw::symmetric_tag_pull(self.handle, bytes.as_mut_ptr(), bytes.len()) }
            .expect("symmetric_tag_pull should not fail for valid tag handle and buffer");
        self.closed = true;
        bytes
    }

    pub fn verify(self, expected: impl AsRef<[u8]>) -> Result<(), Error> {
        let expected = expected.as_ref();
        unsafe { raw::symmetric_tag_verify(self.handle, expected.as_ptr(), expected.len()) }
            .map_err(|e| e.into())
    }
}

impl Drop for Tag {
    fn drop(&mut self) {
        if !self.closed {
            let _ = unsafe { raw::symmetric_tag_close(self.handle) };
        }
    }
}
