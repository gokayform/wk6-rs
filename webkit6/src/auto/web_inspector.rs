// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::{ffi, WebViewBase};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "WebKitWebInspector")]
    pub struct WebInspector(Object<ffi::WebKitWebInspector, ffi::WebKitWebInspectorClass>);

    match fn {
        type_ => || ffi::webkit_web_inspector_get_type(),
    }
}

impl WebInspector {
    #[doc(alias = "webkit_web_inspector_attach")]
    pub fn attach(&self) {
        unsafe {
            ffi::webkit_web_inspector_attach(self.to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_web_inspector_close")]
    pub fn close(&self) {
        unsafe {
            ffi::webkit_web_inspector_close(self.to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_web_inspector_detach")]
    pub fn detach(&self) {
        unsafe {
            ffi::webkit_web_inspector_detach(self.to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_web_inspector_get_attached_height")]
    #[doc(alias = "get_attached_height")]
    #[doc(alias = "attached-height")]
    pub fn attached_height(&self) -> u32 {
        unsafe { ffi::webkit_web_inspector_get_attached_height(self.to_glib_none().0) }
    }

    #[doc(alias = "webkit_web_inspector_get_can_attach")]
    #[doc(alias = "get_can_attach")]
    #[doc(alias = "can-attach")]
    pub fn can_attach(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_web_inspector_get_can_attach(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_web_inspector_get_inspected_uri")]
    #[doc(alias = "get_inspected_uri")]
    #[doc(alias = "inspected-uri")]
    pub fn inspected_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_web_inspector_get_inspected_uri(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_web_inspector_get_web_view")]
    #[doc(alias = "get_web_view")]
    pub fn web_view(&self) -> Option<WebViewBase> {
        unsafe {
            from_glib_none(ffi::webkit_web_inspector_get_web_view(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_web_inspector_is_attached")]
    pub fn is_attached(&self) -> bool {
        unsafe { from_glib(ffi::webkit_web_inspector_is_attached(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_web_inspector_show")]
    pub fn show(&self) {
        unsafe {
            ffi::webkit_web_inspector_show(self.to_glib_none().0);
        }
    }

    #[doc(alias = "attach")]
    pub fn connect_attach<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn attach_trampoline<F: Fn(&WebInspector) -> bool + 'static>(
            this: *mut ffi::WebKitWebInspector,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"attach\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    attach_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "bring-to-front")]
    pub fn connect_bring_to_front<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn bring_to_front_trampoline<F: Fn(&WebInspector) -> bool + 'static>(
            this: *mut ffi::WebKitWebInspector,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"bring-to-front\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    bring_to_front_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "closed")]
    pub fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<F: Fn(&WebInspector) + 'static>(
            this: *mut ffi::WebKitWebInspector,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"closed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    closed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "detach")]
    pub fn connect_detach<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn detach_trampoline<F: Fn(&WebInspector) -> bool + 'static>(
            this: *mut ffi::WebKitWebInspector,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"detach\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    detach_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "open-window")]
    pub fn connect_open_window<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn open_window_trampoline<F: Fn(&WebInspector) -> bool + 'static>(
            this: *mut ffi::WebKitWebInspector,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"open-window\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    open_window_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "attached-height")]
    pub fn connect_attached_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_attached_height_trampoline<F: Fn(&WebInspector) + 'static>(
            this: *mut ffi::WebKitWebInspector,
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
                b"notify::attached-height\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_attached_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "can-attach")]
    pub fn connect_can_attach_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_attach_trampoline<F: Fn(&WebInspector) + 'static>(
            this: *mut ffi::WebKitWebInspector,
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
                b"notify::can-attach\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_can_attach_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "inspected-uri")]
    pub fn connect_inspected_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inspected_uri_trampoline<F: Fn(&WebInspector) + 'static>(
            this: *mut ffi::WebKitWebInspector,
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
                b"notify::inspected-uri\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_inspected_uri_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
