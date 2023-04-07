// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitHitTestResult")]
    pub struct HitTestResult(Object<ffi::WebKitHitTestResult, ffi::WebKitHitTestResultClass>);

    match fn {
        type_ => || ffi::webkit_hit_test_result_get_type(),
    }
}

impl HitTestResult {
    #[doc(alias = "webkit_hit_test_result_context_is_editable")]
    pub fn context_is_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_editable(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_hit_test_result_context_is_image")]
    pub fn context_is_image(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_image(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_hit_test_result_context_is_link")]
    pub fn context_is_link(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_link(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_hit_test_result_context_is_media")]
    pub fn context_is_media(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_media(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_hit_test_result_context_is_scrollbar")]
    pub fn context_is_scrollbar(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_scrollbar(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_hit_test_result_context_is_selection")]
    pub fn context_is_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_selection(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_hit_test_result_get_context")]
    #[doc(alias = "get_context")]
    pub fn context(&self) -> u32 {
        unsafe { ffi::webkit_hit_test_result_get_context(self.to_glib_none().0) }
    }

    #[doc(alias = "webkit_hit_test_result_get_image_uri")]
    #[doc(alias = "get_image_uri")]
    pub fn image_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_image_uri(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_hit_test_result_get_link_label")]
    #[doc(alias = "get_link_label")]
    pub fn link_label(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_link_label(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_hit_test_result_get_link_title")]
    #[doc(alias = "get_link_title")]
    pub fn link_title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_link_title(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_hit_test_result_get_link_uri")]
    #[doc(alias = "get_link_uri")]
    pub fn link_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_link_uri(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_hit_test_result_get_media_uri")]
    #[doc(alias = "get_media_uri")]
    pub fn media_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_media_uri(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for HitTestResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("HitTestResult")
    }
}