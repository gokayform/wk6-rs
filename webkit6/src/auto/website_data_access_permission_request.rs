// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::PermissionRequest;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitWebsiteDataAccessPermissionRequest")]
    pub struct WebsiteDataAccessPermissionRequest(Object<ffi::WebKitWebsiteDataAccessPermissionRequest, ffi::WebKitWebsiteDataAccessPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        type_ => || ffi::webkit_website_data_access_permission_request_get_type(),
    }
}

impl WebsiteDataAccessPermissionRequest {
    #[doc(alias = "webkit_website_data_access_permission_request_get_current_domain")]
    #[doc(alias = "get_current_domain")]
    pub fn current_domain(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(
                ffi::webkit_website_data_access_permission_request_get_current_domain(
                    self.to_glib_none().0,
                ),
            )
        }
    }

    #[doc(alias = "webkit_website_data_access_permission_request_get_requesting_domain")]
    #[doc(alias = "get_requesting_domain")]
    pub fn requesting_domain(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(
                ffi::webkit_website_data_access_permission_request_get_requesting_domain(
                    self.to_glib_none().0,
                ),
            )
        }
    }
}

impl fmt::Display for WebsiteDataAccessPermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebsiteDataAccessPermissionRequest")
    }
}
