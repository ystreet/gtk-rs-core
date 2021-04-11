// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::SocketConnection;
use crate::SocketListener;
use crate::SocketService;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct ThreadedSocketService(Object<ffi::GThreadedSocketService, ffi::GThreadedSocketServiceClass>) @extends SocketService, SocketListener;

    match fn {
        get_type => || ffi::g_threaded_socket_service_get_type(),
    }
}

pub const NONE_THREADED_SOCKET_SERVICE: Option<&ThreadedSocketService> = None;

pub trait ThreadedSocketServiceExt: 'static {
    #[doc(alias = "get_property_max_threads")]
    fn max_threads(&self) -> i32;

    fn connect_run<F: Fn(&Self, &SocketConnection, Option<&glib::Object>) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<ThreadedSocketService>> ThreadedSocketServiceExt for O {
    fn max_threads(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"max-threads\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-threads` getter")
                .unwrap()
        }
    }

    fn connect_run<F: Fn(&Self, &SocketConnection, Option<&glib::Object>) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn run_trampoline<
            P,
            F: Fn(&P, &SocketConnection, Option<&glib::Object>) -> bool + 'static,
        >(
            this: *mut ffi::GThreadedSocketService,
            connection: *mut ffi::GSocketConnection,
            source_object: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean
        where
            P: IsA<ThreadedSocketService>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ThreadedSocketService::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(connection),
                Option::<glib::Object>::from_glib_borrow(source_object)
                    .as_ref()
                    .as_ref(),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"run\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    run_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ThreadedSocketService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ThreadedSocketService")
    }
}
