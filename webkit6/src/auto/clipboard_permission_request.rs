// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::PermissionRequest;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitClipboardPermissionRequest")]
    pub struct ClipboardPermissionRequest(Object<ffi::WebKitClipboardPermissionRequest, ffi::WebKitClipboardPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        type_ => || ffi::webkit_clipboard_permission_request_get_type(),
    }
}

impl ClipboardPermissionRequest {}

impl fmt::Display for ClipboardPermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ClipboardPermissionRequest")
    }
}
