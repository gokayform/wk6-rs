// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::ScriptWorld;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "WebKitFrame")]
    pub struct Frame(Object<ffi::WebKitFrame, ffi::WebKitFrameClass>);

    match fn {
        type_ => || ffi::webkit_frame_get_type(),
    }
}

impl Frame {
    #[doc(alias = "webkit_frame_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&self) -> u64 {
        unsafe { ffi::webkit_frame_get_id(self.to_glib_none().0) }
    }

    #[doc(alias = "webkit_frame_get_js_context")]
    #[doc(alias = "get_js_context")]
    pub fn js_context(&self) -> Option<javascriptcore::Context> {
        unsafe { from_glib_full(ffi::webkit_frame_get_js_context(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_frame_get_js_context_for_script_world")]
    #[doc(alias = "get_js_context_for_script_world")]
    pub fn js_context_for_script_world(
        &self,
        world: &ScriptWorld,
    ) -> Option<javascriptcore::Context> {
        unsafe {
            from_glib_full(ffi::webkit_frame_get_js_context_for_script_world(
                self.to_glib_none().0,
                world.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_frame_get_uri")]
    #[doc(alias = "get_uri")]
    pub fn uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_frame_get_uri(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_frame_is_main_frame")]
    pub fn is_main_frame(&self) -> bool {
        unsafe { from_glib(ffi::webkit_frame_is_main_frame(self.to_glib_none().0)) }
    }
}
