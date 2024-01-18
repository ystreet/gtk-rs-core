// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v2_60")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
use crate::ResolverNameLookupFlags;
use crate::{AsyncResult, Cancellable, InetAddress, ResolverRecordType, SrvTarget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, pin::Pin};

glib::wrapper! {
    #[doc(alias = "GResolver")]
    pub struct Resolver(Object<ffi::GResolver, ffi::GResolverClass>);

    match fn {
        type_ => || ffi::g_resolver_get_type(),
    }
}

impl Resolver {
    pub const NONE: Option<&'static Resolver> = None;

    //#[doc(alias = "g_resolver_free_addresses")]
    //pub fn free_addresses(addresses: /*Unimplemented*/&[&Basic: Pointer]) {
    //    unsafe { TODO: call ffi:g_resolver_free_addresses() }
    //}

    //#[doc(alias = "g_resolver_free_targets")]
    //pub fn free_targets(targets: /*Unimplemented*/&[&Basic: Pointer]) {
    //    unsafe { TODO: call ffi:g_resolver_free_targets() }
    //}

    #[doc(alias = "g_resolver_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Resolver {
        unsafe { from_glib_full(ffi::g_resolver_get_default()) }
    }

    //#[doc(alias = "g_resolver_records_from_res_query")]
    //pub fn records_from_res_query(rrname: &str, rrtype: i32, answer: u8, len: isize, herr: i32) -> Result</*Unimplemented*/Vec<Basic: Pointer>, glib::Error> {
    //    unsafe { TODO: call ffi:g_resolver_records_from_res_query() }
    //}
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Resolver>> Sealed for T {}
}

pub trait ResolverExt: IsA<Resolver> + sealed::Sealed + 'static {
    #[cfg(feature = "v2_78")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_78")))]
    #[doc(alias = "g_resolver_get_timeout")]
    #[doc(alias = "get_timeout")]
    fn timeout(&self) -> u32 {
        unsafe { ffi::g_resolver_get_timeout(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "g_resolver_lookup_by_address")]
    fn lookup_by_address(
        &self,
        address: &impl IsA<InetAddress>,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_address(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_resolver_lookup_by_address_async")]
    fn lookup_by_address_async<P: FnOnce(Result<glib::GString, glib::Error>) + 'static>(
        &self,
        address: &impl IsA<InetAddress>,
        cancellable: Option<&impl IsA<Cancellable>>,
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
        unsafe extern "C" fn lookup_by_address_async_trampoline<
            P: FnOnce(Result<glib::GString, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::g_resolver_lookup_by_address_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = lookup_by_address_async_trampoline::<P>;
        unsafe {
            ffi::g_resolver_lookup_by_address_async(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn lookup_by_address_future(
        &self,
        address: &(impl IsA<InetAddress> + Clone + 'static),
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::GString, glib::Error>> + 'static>>
    {
        let address = address.clone();
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.lookup_by_address_async(&address, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[doc(alias = "g_resolver_lookup_by_name")]
    fn lookup_by_name(
        &self,
        hostname: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Vec<InetAddress>, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_name(
                self.as_ref().to_glib_none().0,
                hostname.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_resolver_lookup_by_name_async")]
    fn lookup_by_name_async<P: FnOnce(Result<Vec<InetAddress>, glib::Error>) + 'static>(
        &self,
        hostname: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
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
        unsafe extern "C" fn lookup_by_name_async_trampoline<
            P: FnOnce(Result<Vec<InetAddress>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::g_resolver_lookup_by_name_finish(_source_object as *mut _, res, &mut error);
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
        let callback = lookup_by_name_async_trampoline::<P>;
        unsafe {
            ffi::g_resolver_lookup_by_name_async(
                self.as_ref().to_glib_none().0,
                hostname.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn lookup_by_name_future(
        &self,
        hostname: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<InetAddress>, glib::Error>> + 'static>>
    {
        let hostname = String::from(hostname);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.lookup_by_name_async(&hostname, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[cfg(feature = "v2_60")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_resolver_lookup_by_name_with_flags")]
    fn lookup_by_name_with_flags(
        &self,
        hostname: &str,
        flags: ResolverNameLookupFlags,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Vec<InetAddress>, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_name_with_flags(
                self.as_ref().to_glib_none().0,
                hostname.to_glib_none().0,
                flags.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v2_60")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_resolver_lookup_by_name_with_flags_async")]
    fn lookup_by_name_with_flags_async<
        P: FnOnce(Result<Vec<InetAddress>, glib::Error>) + 'static,
    >(
        &self,
        hostname: &str,
        flags: ResolverNameLookupFlags,
        cancellable: Option<&impl IsA<Cancellable>>,
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
        unsafe extern "C" fn lookup_by_name_with_flags_async_trampoline<
            P: FnOnce(Result<Vec<InetAddress>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_resolver_lookup_by_name_with_flags_finish(
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
        let callback = lookup_by_name_with_flags_async_trampoline::<P>;
        unsafe {
            ffi::g_resolver_lookup_by_name_with_flags_async(
                self.as_ref().to_glib_none().0,
                hostname.to_glib_none().0,
                flags.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "v2_60")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
    fn lookup_by_name_with_flags_future(
        &self,
        hostname: &str,
        flags: ResolverNameLookupFlags,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<InetAddress>, glib::Error>> + 'static>>
    {
        let hostname = String::from(hostname);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.lookup_by_name_with_flags_async(
                    &hostname,
                    flags,
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    #[doc(alias = "g_resolver_lookup_records")]
    fn lookup_records(
        &self,
        rrname: &str,
        record_type: ResolverRecordType,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Vec<glib::Variant>, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_resolver_lookup_records(
                self.as_ref().to_glib_none().0,
                rrname.to_glib_none().0,
                record_type.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_resolver_lookup_records_async")]
    fn lookup_records_async<P: FnOnce(Result<Vec<glib::Variant>, glib::Error>) + 'static>(
        &self,
        rrname: &str,
        record_type: ResolverRecordType,
        cancellable: Option<&impl IsA<Cancellable>>,
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
        unsafe extern "C" fn lookup_records_async_trampoline<
            P: FnOnce(Result<Vec<glib::Variant>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::g_resolver_lookup_records_finish(_source_object as *mut _, res, &mut error);
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
        let callback = lookup_records_async_trampoline::<P>;
        unsafe {
            ffi::g_resolver_lookup_records_async(
                self.as_ref().to_glib_none().0,
                rrname.to_glib_none().0,
                record_type.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn lookup_records_future(
        &self,
        rrname: &str,
        record_type: ResolverRecordType,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<Vec<glib::Variant>, glib::Error>> + 'static>,
    > {
        let rrname = String::from(rrname);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.lookup_records_async(&rrname, record_type, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[doc(alias = "g_resolver_lookup_service")]
    fn lookup_service(
        &self,
        service: &str,
        protocol: &str,
        domain: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Vec<SrvTarget>, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_resolver_lookup_service(
                self.as_ref().to_glib_none().0,
                service.to_glib_none().0,
                protocol.to_glib_none().0,
                domain.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_resolver_lookup_service_async")]
    fn lookup_service_async<P: FnOnce(Result<Vec<SrvTarget>, glib::Error>) + 'static>(
        &self,
        service: &str,
        protocol: &str,
        domain: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
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
        unsafe extern "C" fn lookup_service_async_trampoline<
            P: FnOnce(Result<Vec<SrvTarget>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::g_resolver_lookup_service_finish(_source_object as *mut _, res, &mut error);
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
        let callback = lookup_service_async_trampoline::<P>;
        unsafe {
            ffi::g_resolver_lookup_service_async(
                self.as_ref().to_glib_none().0,
                service.to_glib_none().0,
                protocol.to_glib_none().0,
                domain.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn lookup_service_future(
        &self,
        service: &str,
        protocol: &str,
        domain: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<SrvTarget>, glib::Error>> + 'static>>
    {
        let service = String::from(service);
        let protocol = String::from(protocol);
        let domain = String::from(domain);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.lookup_service_async(
                    &service,
                    &protocol,
                    &domain,
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    #[doc(alias = "g_resolver_set_default")]
    fn set_default(&self) {
        unsafe {
            ffi::g_resolver_set_default(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_78")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_78")))]
    #[doc(alias = "g_resolver_set_timeout")]
    fn set_timeout(&self, timeout_ms: u32) {
        unsafe {
            ffi::g_resolver_set_timeout(self.as_ref().to_glib_none().0, timeout_ms);
        }
    }

    #[doc(alias = "reload")]
    fn connect_reload<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn reload_trampoline<P: IsA<Resolver>, F: Fn(&P) + 'static>(
            this: *mut ffi::GResolver,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Resolver::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"reload\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    reload_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v2_78")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_78")))]
    #[doc(alias = "timeout")]
    fn connect_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<P: IsA<Resolver>, F: Fn(&P) + 'static>(
            this: *mut ffi::GResolver,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Resolver::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Resolver>> ResolverExt for O {}
