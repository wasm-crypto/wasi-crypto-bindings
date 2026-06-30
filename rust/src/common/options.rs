use std::marker::PhantomData;

use super::algorithm_type::AlgorithmType;
use crate::error::Error;
use crate::raw;

#[derive(Debug)]
pub struct Options<T: AlgorithmType> {
    pub(crate) handle: raw::Options,
    _t: PhantomData<T>,
}

impl<T: AlgorithmType> Options<T> {
    pub fn new() -> Self {
        let handle = unsafe { raw::options_open(T::RAW) }
            .expect("options_open should not fail for valid algorithm types");
        Options {
            handle,
            _t: PhantomData,
        }
    }

    pub fn set(&mut self, name: &'static str, value: impl AsRef<[u8]>) -> Result<(), Error> {
        let value = value.as_ref();
        unsafe { raw::options_set(self.handle, name, value.as_ptr(), value.len()) }
            .map_err(|e| e.into())
    }

    pub fn set_u64(&mut self, name: &'static str, value: u64) -> Result<(), Error> {
        unsafe { raw::options_set_u64(self.handle, name, value) }.map_err(|e| e.into())
    }
}

impl<T: AlgorithmType> Default for Options<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: AlgorithmType> Drop for Options<T> {
    fn drop(&mut self) {
        let _ = unsafe { raw::options_close(self.handle) };
    }
}

pub(crate) struct OptOptions;

impl OptOptions {
    pub fn none() -> raw::OptOptions {
        raw::OptOptions {
            tag: raw::OPT_OPTIONS_U_NONE.raw(),
            u: raw::OptOptionsUnion { none: () },
        }
    }

    pub fn some<T: AlgorithmType>(options: &Options<T>) -> raw::OptOptions {
        raw::OptOptions {
            tag: raw::OPT_OPTIONS_U_SOME.raw(),
            u: raw::OptOptionsUnion {
                some: options.handle,
            },
        }
    }
}
