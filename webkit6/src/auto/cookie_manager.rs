// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::{CookieAcceptPolicy, CookiePersistentStorage};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "WebKitCookieManager")]
    pub struct CookieManager(Object<ffi::WebKitCookieManager, ffi::WebKitCookieManagerClass>);

    match fn {
        type_ => || ffi::webkit_cookie_manager_get_type(),
    }
}

impl CookieManager {
    #[doc(alias = "webkit_cookie_manager_add_cookie")]
    pub fn add_cookie<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cookie: &soup::Cookie,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn add_cookie_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_cookie_manager_add_cookie_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = add_cookie_trampoline::<P>;
        unsafe {
            ffi::webkit_cookie_manager_add_cookie(
                self.to_glib_none().0,
                mut_override(cookie.to_glib_none().0),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn add_cookie_future(
        &self,
        cookie: &soup::Cookie,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let cookie = cookie.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.add_cookie(&cookie, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "webkit_cookie_manager_delete_cookie")]
    pub fn delete_cookie<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cookie: &soup::Cookie,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn delete_cookie_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_cookie_manager_delete_cookie_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = delete_cookie_trampoline::<P>;
        unsafe {
            ffi::webkit_cookie_manager_delete_cookie(
                self.to_glib_none().0,
                mut_override(cookie.to_glib_none().0),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn delete_cookie_future(
        &self,
        cookie: &soup::Cookie,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let cookie = cookie.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.delete_cookie(&cookie, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "webkit_cookie_manager_get_accept_policy")]
    #[doc(alias = "get_accept_policy")]
    pub fn accept_policy<P: FnOnce(Result<CookieAcceptPolicy, glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn accept_policy_trampoline<
            P: FnOnce(Result<CookieAcceptPolicy, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_cookie_manager_get_accept_policy_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = accept_policy_trampoline::<P>;
        unsafe {
            ffi::webkit_cookie_manager_get_accept_policy(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn accept_policy_future(
        &self,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<CookieAcceptPolicy, glib::Error>> + 'static>,
    > {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.accept_policy(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[cfg(feature = "v2_42")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_42")))]
    #[doc(alias = "webkit_cookie_manager_get_all_cookies")]
    #[doc(alias = "get_all_cookies")]
    pub fn all_cookies<P: FnOnce(Result<Vec<soup::Cookie>, glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn all_cookies_trampoline<
            P: FnOnce(Result<Vec<soup::Cookie>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_cookie_manager_get_all_cookies_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = all_cookies_trampoline::<P>;
        unsafe {
            ffi::webkit_cookie_manager_get_all_cookies(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "v2_42")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_42")))]
    pub fn all_cookies_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<soup::Cookie>, glib::Error>> + 'static>>
    {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.all_cookies(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "webkit_cookie_manager_get_cookies")]
    #[doc(alias = "get_cookies")]
    pub fn cookies<P: FnOnce(Result<Vec<soup::Cookie>, glib::Error>) + 'static>(
        &self,
        uri: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn cookies_trampoline<
            P: FnOnce(Result<Vec<soup::Cookie>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_cookie_manager_get_cookies_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = cookies_trampoline::<P>;
        unsafe {
            ffi::webkit_cookie_manager_get_cookies(
                self.to_glib_none().0,
                uri.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn cookies_future(
        &self,
        uri: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<soup::Cookie>, glib::Error>> + 'static>>
    {
        let uri = String::from(uri);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.cookies(&uri, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[cfg(feature = "v2_42")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_42")))]
    #[doc(alias = "webkit_cookie_manager_replace_cookies")]
    pub fn replace_cookies<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cookies: &[soup::Cookie],
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn replace_cookies_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_cookie_manager_replace_cookies_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = replace_cookies_trampoline::<P>;
        unsafe {
            ffi::webkit_cookie_manager_replace_cookies(
                self.to_glib_none().0,
                cookies.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "v2_42")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_42")))]
    pub fn replace_cookies_future(
        &self,
        cookies: &[soup::Cookie],
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let cookies = cookies.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.replace_cookies(&cookies, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "webkit_cookie_manager_set_accept_policy")]
    pub fn set_accept_policy(&self, policy: CookieAcceptPolicy) {
        unsafe {
            ffi::webkit_cookie_manager_set_accept_policy(self.to_glib_none().0, policy.into_glib());
        }
    }

    #[doc(alias = "webkit_cookie_manager_set_persistent_storage")]
    pub fn set_persistent_storage(&self, filename: &str, storage: CookiePersistentStorage) {
        unsafe {
            ffi::webkit_cookie_manager_set_persistent_storage(
                self.to_glib_none().0,
                filename.to_glib_none().0,
                storage.into_glib(),
            );
        }
    }

    #[doc(alias = "changed")]
    pub fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&CookieManager) + 'static>(
            this: *mut ffi::WebKitCookieManager,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CookieManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CookieManager")
    }
}
