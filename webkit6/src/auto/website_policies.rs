// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::AutoplayPolicy;
use glib::{prelude::*, translate::*};

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

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`WebsitePolicies`] objects.
    ///
    /// This method returns an instance of [`WebsitePoliciesBuilder`](crate::builders::WebsitePoliciesBuilder) which can be used to create [`WebsitePolicies`] objects.
    pub fn builder() -> WebsitePoliciesBuilder {
        WebsitePoliciesBuilder::new()
    }

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
        ObjectExt::property(self, "autoplay")
    }
}

impl Default for WebsitePolicies {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`WebsitePolicies`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct WebsitePoliciesBuilder {
    builder: glib::object::ObjectBuilder<'static, WebsitePolicies>,
}

impl WebsitePoliciesBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn autoplay(self, autoplay: AutoplayPolicy) -> Self {
        Self {
            builder: self.builder.property("autoplay", autoplay),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`WebsitePolicies`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> WebsitePolicies {
        self.builder.build()
    }
}
