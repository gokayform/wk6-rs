#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! assert_initialized_main_thread {
    () => {};
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
pub use gtk;
pub use javascriptcore;
pub use soup;

#[allow(unused_imports)]
mod auto;
pub use crate::auto::*;

pub mod prelude {
    pub use super::auto::traits::*;
}
