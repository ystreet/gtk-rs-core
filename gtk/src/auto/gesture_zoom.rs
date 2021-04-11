// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::Gesture;
use crate::PropagationPhase;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct GestureZoom(Object<ffi::GtkGestureZoom, ffi::GtkGestureZoomClass>) @extends Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_zoom_get_type(),
    }
}

impl GestureZoom {
    #[doc(alias = "gtk_gesture_zoom_new")]
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureZoom {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_zoom_new(widget.as_ref().to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_gesture_zoom_get_scale_delta")]
    pub fn scale_delta(&self) -> f64 {
        unsafe { ffi::gtk_gesture_zoom_get_scale_delta(self.to_glib_none().0) }
    }

    pub fn connect_scale_changed<F: Fn(&GestureZoom, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn scale_changed_trampoline<F: Fn(&GestureZoom, f64) + 'static>(
            this: *mut ffi::GtkGestureZoom,
            scale: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), scale)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scale-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scale_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct GestureZoomBuilder {
    n_points: Option<u32>,
    window: Option<gdk::Window>,
    propagation_phase: Option<PropagationPhase>,
    widget: Option<Widget>,
}

impl GestureZoomBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> GestureZoom {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref n_points) = self.n_points {
            properties.push(("n-points", n_points));
        }
        if let Some(ref window) = self.window {
            properties.push(("window", window));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        if let Some(ref widget) = self.widget {
            properties.push(("widget", widget));
        }
        let ret = glib::Object::new::<GestureZoom>(&properties).expect("object new");
        ret
    }

    pub fn n_points(mut self, n_points: u32) -> Self {
        self.n_points = Some(n_points);
        self
    }

    pub fn window(mut self, window: &gdk::Window) -> Self {
        self.window = Some(window.clone());
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }

    pub fn widget<P: IsA<Widget>>(mut self, widget: &P) -> Self {
        self.widget = Some(widget.clone().upcast());
        self
    }
}

impl fmt::Display for GestureZoom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GestureZoom")
    }
}
