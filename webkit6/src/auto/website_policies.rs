// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::AutoplayPolicy;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitWebsitePolicies")]
    pub struct WebsitePolicies(Object<ffi::WebKitWebsitePolicies, ffi::WebKitWebsitePoliciesClass>);

    match fn {
        type_ => || ffi::webkit_website_policies_get_type(),
    }
}

impl WebsitePolicies {
    #[doc(alias = "webkit_website_policies_new")]
    pub fn new() -> WebsitePolicies {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::webkit_website_policies_new()) }
    }

    //#[doc(alias = "webkit_website_policies_new_with_policies")]
    //#[doc(alias = "new_with_policies")]
    //pub fn with_policies(first_policy_name: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> WebsitePolicies {
    //    unsafe { TODO: call ffi:webkit_website_policies_new_with_policies() }
    //}

    #[doc(alias = "webkit_website_policies_get_autoplay_policy")]
    #[doc(alias = "get_autoplay_policy")]
    pub fn autoplay_policy(&self) -> AutoplayPolicy {
        unsafe {
            from_glib(ffi::webkit_website_policies_get_autoplay_policy(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn autoplay(&self) -> AutoplayPolicy {
        glib::ObjectExt::property(self, "autoplay")
    }
}

impl Default for WebsitePolicies {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for WebsitePolicies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebsitePolicies")
    }
}