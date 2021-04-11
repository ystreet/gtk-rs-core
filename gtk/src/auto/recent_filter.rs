// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buildable;
use crate::RecentFilterFlags;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct RecentFilter(Object<ffi::GtkRecentFilter>) @implements Buildable;

    match fn {
        get_type => || ffi::gtk_recent_filter_get_type(),
    }
}

impl RecentFilter {
    #[doc(alias = "gtk_recent_filter_new")]
    pub fn new() -> RecentFilter {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_recent_filter_new()) }
    }

    #[doc(alias = "gtk_recent_filter_add_age")]
    pub fn add_age(&self, days: i32) {
        unsafe {
            ffi::gtk_recent_filter_add_age(self.to_glib_none().0, days);
        }
    }

    #[doc(alias = "gtk_recent_filter_add_application")]
    pub fn add_application(&self, application: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_application(
                self.to_glib_none().0,
                application.to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "gtk_recent_filter_add_custom")]
    //pub fn add_custom(&self, needed: RecentFilterFlags, func: /*Unimplemented*/Fn(/*Ignored*/RecentFilterInfo) -> bool, data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gtk_recent_filter_add_custom() }
    //}

    #[doc(alias = "gtk_recent_filter_add_group")]
    pub fn add_group(&self, group: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_group(self.to_glib_none().0, group.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_recent_filter_add_mime_type")]
    pub fn add_mime_type(&self, mime_type: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_mime_type(self.to_glib_none().0, mime_type.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_recent_filter_add_pattern")]
    pub fn add_pattern(&self, pattern: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_recent_filter_add_pixbuf_formats")]
    pub fn add_pixbuf_formats(&self) {
        unsafe {
            ffi::gtk_recent_filter_add_pixbuf_formats(self.to_glib_none().0);
        }
    }

    //#[doc(alias = "gtk_recent_filter_filter")]
    //pub fn filter(&self, filter_info: /*Ignored*/&RecentFilterInfo) -> bool {
    //    unsafe { TODO: call ffi:gtk_recent_filter_filter() }
    //}

    #[doc(alias = "gtk_recent_filter_get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_recent_filter_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_recent_filter_get_needed")]
    pub fn needed(&self) -> RecentFilterFlags {
        unsafe { from_glib(ffi::gtk_recent_filter_get_needed(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_recent_filter_set_name")]
    pub fn set_name(&self, name: &str) {
        unsafe {
            ffi::gtk_recent_filter_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }
}

impl Default for RecentFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RecentFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RecentFilter")
    }
}
