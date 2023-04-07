// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::{CheckSyntaxMode, CheckSyntaxResult, Exception, Value, VirtualMachine};
use glib::translate::*;
use std::{boxed::Box as Box_, fmt, ptr};

glib::wrapper! {
    #[doc(alias = "JSCContext")]
    pub struct Context(Object<ffi::JSCContext, ffi::JSCContextClass>);

    match fn {
        type_ => || ffi::jsc_context_get_type(),
    }
}

impl Context {
    #[doc(alias = "jsc_context_new")]
    pub fn new() -> Context {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::jsc_context_new()) }
    }

    #[doc(alias = "jsc_context_new_with_virtual_machine")]
    #[doc(alias = "new_with_virtual_machine")]
    pub fn with_virtual_machine(vm: &VirtualMachine) -> Context {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::jsc_context_new_with_virtual_machine(
                vm.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "jsc_context_check_syntax")]
    pub fn check_syntax(
        &self,
        code: &str,
        mode: CheckSyntaxMode,
        uri: &str,
        line_number: u32,
    ) -> (CheckSyntaxResult, Exception) {
        let length = code.len() as _;
        unsafe {
            let mut exception = ptr::null_mut();
            let ret = from_glib(ffi::jsc_context_check_syntax(
                self.to_glib_none().0,
                code.to_glib_none().0,
                length,
                mode.into_glib(),
                uri.to_glib_none().0,
                line_number,
                &mut exception,
            ));
            (ret, from_glib_full(exception))
        }
    }

    #[doc(alias = "jsc_context_clear_exception")]
    pub fn clear_exception(&self) {
        unsafe {
            ffi::jsc_context_clear_exception(self.to_glib_none().0);
        }
    }

    #[doc(alias = "jsc_context_evaluate")]
    pub fn evaluate(&self, code: &str) -> Option<Value> {
        let length = code.len() as _;
        unsafe {
            from_glib_full(ffi::jsc_context_evaluate(
                self.to_glib_none().0,
                code.to_glib_none().0,
                length,
            ))
        }
    }

    //#[doc(alias = "jsc_context_evaluate_in_object")]
    //pub fn evaluate_in_object(&self, code: &str, object_instance: /*Unimplemented*/Option<Basic: Pointer>, object_class: Option<&Class>, uri: &str, line_number: u32) -> (Value, Value) {
    //    unsafe { TODO: call ffi:jsc_context_evaluate_in_object() }
    //}

    #[doc(alias = "jsc_context_evaluate_with_source_uri")]
    pub fn evaluate_with_source_uri(
        &self,
        code: &str,
        uri: &str,
        line_number: u32,
    ) -> Option<Value> {
        let length = code.len() as _;
        unsafe {
            from_glib_full(ffi::jsc_context_evaluate_with_source_uri(
                self.to_glib_none().0,
                code.to_glib_none().0,
                length,
                uri.to_glib_none().0,
                line_number,
            ))
        }
    }

    #[doc(alias = "jsc_context_get_exception")]
    #[doc(alias = "get_exception")]
    pub fn exception(&self) -> Option<Exception> {
        unsafe { from_glib_none(ffi::jsc_context_get_exception(self.to_glib_none().0)) }
    }

    #[doc(alias = "jsc_context_get_global_object")]
    #[doc(alias = "get_global_object")]
    pub fn global_object(&self) -> Option<Value> {
        unsafe { from_glib_full(ffi::jsc_context_get_global_object(self.to_glib_none().0)) }
    }

    #[doc(alias = "jsc_context_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self, name: &str) -> Option<Value> {
        unsafe {
            from_glib_full(ffi::jsc_context_get_value(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "jsc_context_get_virtual_machine")]
    #[doc(alias = "get_virtual_machine")]
    pub fn virtual_machine(&self) -> Option<VirtualMachine> {
        unsafe { from_glib_none(ffi::jsc_context_get_virtual_machine(self.to_glib_none().0)) }
    }

    #[doc(alias = "jsc_context_pop_exception_handler")]
    pub fn pop_exception_handler(&self) {
        unsafe {
            ffi::jsc_context_pop_exception_handler(self.to_glib_none().0);
        }
    }

    #[doc(alias = "jsc_context_push_exception_handler")]
    pub fn push_exception_handler<P: Fn(&Context, &Exception) + 'static>(&self, handler: P) {
        let handler_data: Box_<P> = Box_::new(handler);
        unsafe extern "C" fn handler_func<P: Fn(&Context, &Exception) + 'static>(
            context: *mut ffi::JSCContext,
            exception: *mut ffi::JSCException,
            user_data: glib::ffi::gpointer,
        ) {
            let context = from_glib_borrow(context);
            let exception = from_glib_borrow(exception);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&context, &exception)
        }
        let handler = Some(handler_func::<P> as _);
        unsafe extern "C" fn destroy_notify_func<P: Fn(&Context, &Exception) + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_notify_func::<P> as _);
        let super_callback0: Box_<P> = handler_data;
        unsafe {
            ffi::jsc_context_push_exception_handler(
                self.to_glib_none().0,
                handler,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    //#[doc(alias = "jsc_context_register_class")]
    //pub fn register_class(&self, name: &str, parent_class: Option<&Class>, vtable: /*Ignored*/Option<&mut ClassVTable>) -> Option<Class> {
    //    unsafe { TODO: call ffi:jsc_context_register_class() }
    //}

    #[doc(alias = "jsc_context_set_value")]
    pub fn set_value(&self, name: &str, value: &Value) {
        unsafe {
            ffi::jsc_context_set_value(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "jsc_context_throw")]
    pub fn throw(&self, error_message: &str) {
        unsafe {
            ffi::jsc_context_throw(self.to_glib_none().0, error_message.to_glib_none().0);
        }
    }

    #[doc(alias = "jsc_context_throw_exception")]
    pub fn throw_exception(&self, exception: &Exception) {
        unsafe {
            ffi::jsc_context_throw_exception(self.to_glib_none().0, exception.to_glib_none().0);
        }
    }

    //#[doc(alias = "jsc_context_throw_printf")]
    //pub fn throw_printf(&self, format: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:jsc_context_throw_printf() }
    //}

    #[doc(alias = "jsc_context_throw_with_name")]
    pub fn throw_with_name(&self, error_name: &str, error_message: &str) {
        unsafe {
            ffi::jsc_context_throw_with_name(
                self.to_glib_none().0,
                error_name.to_glib_none().0,
                error_message.to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "jsc_context_throw_with_name_printf")]
    //pub fn throw_with_name_printf(&self, error_name: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:jsc_context_throw_with_name_printf() }
    //}

    #[doc(alias = "jsc_context_get_current")]
    #[doc(alias = "get_current")]
    pub fn current() -> Option<Context> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::jsc_context_get_current()) }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Context")
    }
}
