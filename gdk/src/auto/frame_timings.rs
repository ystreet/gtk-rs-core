// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FrameTimings(Shared<ffi::GdkFrameTimings>);

    match fn {
        ref => |ptr| ffi::gdk_frame_timings_ref(ptr),
        unref => |ptr| ffi::gdk_frame_timings_unref(ptr),
        get_type => || ffi::gdk_frame_timings_get_type(),
    }
}

impl FrameTimings {
    #[doc(alias = "gdk_frame_timings_get_complete")]
    pub fn is_complete(&self) -> bool {
        unsafe { from_glib(ffi::gdk_frame_timings_get_complete(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_frame_timings_get_frame_counter")]
    pub fn frame_counter(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_frame_counter(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_frame_timings_get_frame_time")]
    pub fn frame_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_timings_get_frame_time(self.to_glib_none().0) }
    }
}
