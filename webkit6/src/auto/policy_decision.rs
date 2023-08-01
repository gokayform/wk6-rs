// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::WebsitePolicies;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitPolicyDecision")]
    pub struct PolicyDecision(Object<ffi::WebKitPolicyDecision, ffi::WebKitPolicyDecisionClass>);

    match fn {
        type_ => || ffi::webkit_policy_decision_get_type(),
    }
}

impl PolicyDecision {
    pub const NONE: Option<&'static PolicyDecision> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::PolicyDecision>> Sealed for T {}
}

pub trait PolicyDecisionExt: IsA<PolicyDecision> + sealed::Sealed + 'static {
    #[doc(alias = "webkit_policy_decision_download")]
    fn download(&self) {
        unsafe {
            ffi::webkit_policy_decision_download(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_policy_decision_ignore")]
    fn ignore(&self) {
        unsafe {
            ffi::webkit_policy_decision_ignore(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_policy_decision_use")]
    #[doc(alias = "use")]
    fn use_(&self) {
        unsafe {
            ffi::webkit_policy_decision_use(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_policy_decision_use_with_policies")]
    fn use_with_policies(&self, policies: &WebsitePolicies) {
        unsafe {
            ffi::webkit_policy_decision_use_with_policies(
                self.as_ref().to_glib_none().0,
                policies.to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<PolicyDecision>> PolicyDecisionExt for O {}

impl fmt::Display for PolicyDecision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PolicyDecision")
    }
}
