// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "WebKitURIRequest")]
    pub struct URIRequest(Object<ffi::WebKitURIRequest, ffi::WebKitURIRequestClass>);

    match fn {
        type_ => || ffi::webkit_uri_request_get_type(),
    }
}

impl URIRequest {
    #[doc(alias = "webkit_uri_request_new")]
    pub fn new(uri: &str) -> URIRequest {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::webkit_uri_request_new(uri.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_uri_request_get_http_headers")]
    #[doc(alias = "get_http_headers")]
    pub fn http_headers(&self) -> Option<soup::MessageHeaders> {
        unsafe {
            from_glib_none(ffi::webkit_uri_request_get_http_headers(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_uri_request_get_http_method")]
    #[doc(alias = "get_http_method")]
    pub fn http_method(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_uri_request_get_http_method(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_uri_request_get_uri")]
    #[doc(alias = "get_uri")]
    pub fn uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_uri_request_get_uri(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_uri_request_set_uri")]
    pub fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::webkit_uri_request_set_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    #[doc(alias = "uri")]
    pub fn connect_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uri_trampoline<F: Fn(&URIRequest) + 'static>(
            this: *mut ffi::WebKitURIRequest,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::uri\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_uri_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
