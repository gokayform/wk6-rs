// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::{ffi, AuthenticationScheme, Credential, SecurityOrigin};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "WebKitAuthenticationRequest")]
    pub struct AuthenticationRequest(Object<ffi::WebKitAuthenticationRequest, ffi::WebKitAuthenticationRequestClass>);

    match fn {
        type_ => || ffi::webkit_authentication_request_get_type(),
    }
}

impl AuthenticationRequest {
    #[doc(alias = "webkit_authentication_request_authenticate")]
    pub fn authenticate(&self, credential: Option<&Credential>) {
        unsafe {
            ffi::webkit_authentication_request_authenticate(
                self.to_glib_none().0,
                mut_override(credential.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "webkit_authentication_request_can_save_credentials")]
    pub fn can_save_credentials(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_can_save_credentials(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_authentication_request_cancel")]
    pub fn cancel(&self) {
        unsafe {
            ffi::webkit_authentication_request_cancel(self.to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_authentication_request_get_certificate_pin_flags")]
    #[doc(alias = "get_certificate_pin_flags")]
    pub fn certificate_pin_flags(&self) -> gio::TlsPasswordFlags {
        unsafe {
            from_glib(
                ffi::webkit_authentication_request_get_certificate_pin_flags(self.to_glib_none().0),
            )
        }
    }

    #[doc(alias = "webkit_authentication_request_get_host")]
    #[doc(alias = "get_host")]
    pub fn host(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_authentication_request_get_host(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_authentication_request_get_port")]
    #[doc(alias = "get_port")]
    pub fn port(&self) -> u32 {
        unsafe { ffi::webkit_authentication_request_get_port(self.to_glib_none().0) }
    }

    #[doc(alias = "webkit_authentication_request_get_proposed_credential")]
    #[doc(alias = "get_proposed_credential")]
    pub fn proposed_credential(&self) -> Option<Credential> {
        unsafe {
            from_glib_full(ffi::webkit_authentication_request_get_proposed_credential(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_authentication_request_get_realm")]
    #[doc(alias = "get_realm")]
    pub fn realm(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_authentication_request_get_realm(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_authentication_request_get_scheme")]
    #[doc(alias = "get_scheme")]
    pub fn scheme(&self) -> AuthenticationScheme {
        unsafe {
            from_glib(ffi::webkit_authentication_request_get_scheme(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_authentication_request_get_security_origin")]
    #[doc(alias = "get_security_origin")]
    pub fn security_origin(&self) -> Option<SecurityOrigin> {
        unsafe {
            from_glib_full(ffi::webkit_authentication_request_get_security_origin(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_authentication_request_is_for_proxy")]
    pub fn is_for_proxy(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_is_for_proxy(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_authentication_request_is_retry")]
    pub fn is_retry(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_is_retry(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_authentication_request_set_can_save_credentials")]
    pub fn set_can_save_credentials(&self, enabled: bool) {
        unsafe {
            ffi::webkit_authentication_request_set_can_save_credentials(
                self.to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    #[doc(alias = "webkit_authentication_request_set_proposed_credential")]
    pub fn set_proposed_credential(&self, credential: &mut Credential) {
        unsafe {
            ffi::webkit_authentication_request_set_proposed_credential(
                self.to_glib_none().0,
                credential.to_glib_none_mut().0,
            );
        }
    }

    #[doc(alias = "authenticated")]
    pub fn connect_authenticated<F: Fn(&Self, &Credential) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn authenticated_trampoline<
            F: Fn(&AuthenticationRequest, &Credential) + 'static,
        >(
            this: *mut ffi::WebKitAuthenticationRequest,
            credential: *mut ffi::WebKitCredential,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(credential))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"authenticated\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    authenticated_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "cancelled")]
    pub fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancelled_trampoline<F: Fn(&AuthenticationRequest) + 'static>(
            this: *mut ffi::WebKitAuthenticationRequest,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancelled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    cancelled_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
