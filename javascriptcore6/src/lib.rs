#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! assert_initialized_main_thread {
    () => {};
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub use ffi;
pub use glib;

mod auto;
pub use crate::auto::*;

pub mod functions {
    pub use super::auto::functions::*;
}
