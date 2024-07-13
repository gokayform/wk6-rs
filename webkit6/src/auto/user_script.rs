// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::{ffi, UserContentInjectedFrames, UserScriptInjectionTime};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct UserScript(Shared<ffi::WebKitUserScript>);

    match fn {
        ref => |ptr| ffi::webkit_user_script_ref(ptr),
        unref => |ptr| ffi::webkit_user_script_unref(ptr),
        type_ => || ffi::webkit_user_script_get_type(),
    }
}

impl UserScript {
    #[doc(alias = "webkit_user_script_new")]
    pub fn new(
        source: &str,
        injected_frames: UserContentInjectedFrames,
        injection_time: UserScriptInjectionTime,
        allow_list: &[&str],
        block_list: &[&str],
    ) -> UserScript {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_user_script_new(
                source.to_glib_none().0,
                injected_frames.into_glib(),
                injection_time.into_glib(),
                allow_list.to_glib_none().0,
                block_list.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_user_script_new_for_world")]
    #[doc(alias = "new_for_world")]
    pub fn for_world(
        source: &str,
        injected_frames: UserContentInjectedFrames,
        injection_time: UserScriptInjectionTime,
        world_name: &str,
        allow_list: &[&str],
        block_list: &[&str],
    ) -> UserScript {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_user_script_new_for_world(
                source.to_glib_none().0,
                injected_frames.into_glib(),
                injection_time.into_glib(),
                world_name.to_glib_none().0,
                allow_list.to_glib_none().0,
                block_list.to_glib_none().0,
            ))
        }
    }
}
