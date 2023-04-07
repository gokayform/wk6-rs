// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::{Frame, WebPage};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitScriptWorld")]
    pub struct ScriptWorld(Object<ffi::WebKitScriptWorld, ffi::WebKitScriptWorldClass>);

    match fn {
        type_ => || ffi::webkit_script_world_get_type(),
    }
}

impl ScriptWorld {
    #[doc(alias = "webkit_script_world_new")]
    pub fn new() -> ScriptWorld {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::webkit_script_world_new()) }
    }

    #[doc(alias = "webkit_script_world_new_with_name")]
    #[doc(alias = "new_with_name")]
    pub fn with_name(name: &str) -> ScriptWorld {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_script_world_new_with_name(
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_script_world_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_script_world_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_script_world_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Option<ScriptWorld> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::webkit_script_world_get_default()) }
    }

    #[doc(alias = "window-object-cleared")]
    pub fn connect_window_object_cleared<F: Fn(&Self, &WebPage, &Frame) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn window_object_cleared_trampoline<
            F: Fn(&ScriptWorld, &WebPage, &Frame) + 'static,
        >(
            this: *mut ffi::WebKitScriptWorld,
            page: *mut ffi::WebKitWebPage,
            frame: *mut ffi::WebKitFrame,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(page),
                &from_glib_borrow(frame),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"window-object-cleared\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    window_object_cleared_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ScriptWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ScriptWorld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ScriptWorld")
    }
}