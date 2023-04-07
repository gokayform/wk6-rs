// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitBackForwardListItem")]
    pub struct BackForwardListItem(Object<ffi::WebKitBackForwardListItem, ffi::WebKitBackForwardListItemClass>);

    match fn {
        type_ => || ffi::webkit_back_forward_list_item_get_type(),
    }
}

impl BackForwardListItem {
    #[doc(alias = "webkit_back_forward_list_item_get_original_uri")]
    #[doc(alias = "get_original_uri")]
    pub fn original_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_item_get_original_uri(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_back_forward_list_item_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_item_get_title(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_back_forward_list_item_get_uri")]
    #[doc(alias = "get_uri")]
    pub fn uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_item_get_uri(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for BackForwardListItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BackForwardListItem")
    }
}
