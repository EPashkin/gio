// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use IOStream;
use TlsCertificate;
use TlsCertificateFlags;
use TlsDatabase;
use TlsInteraction;
use TlsRehandshakeMode;
#[cfg(feature = "futures")]
use futures::future;
use gio_sys;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct TlsConnection(Object<gio_sys::GTlsConnection, gio_sys::GTlsConnectionClass, TlsConnectionClass>) @extends IOStream;

    match fn {
        get_type => || gio_sys::g_tls_connection_get_type(),
    }
}

pub const NONE_TLS_CONNECTION: Option<&TlsConnection> = None;

pub trait TlsConnectionExt: 'static {
    fn emit_accept_certificate<P: IsA<TlsCertificate>>(&self, peer_cert: &P, errors: TlsCertificateFlags) -> bool;

    fn get_certificate(&self) -> Option<TlsCertificate>;

    fn get_database(&self) -> Option<TlsDatabase>;

    fn get_interaction(&self) -> Option<TlsInteraction>;

    fn get_peer_certificate(&self) -> Option<TlsCertificate>;

    fn get_peer_certificate_errors(&self) -> TlsCertificateFlags;

    fn get_rehandshake_mode(&self) -> TlsRehandshakeMode;

    fn get_require_close_notify(&self) -> bool;

    fn handshake<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error>;

    fn handshake_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn handshake_async_future(&self, io_priority: glib::Priority) -> Box_<dyn future::Future<Output = Result<(), Error>> + std::marker::Unpin>;

    fn set_certificate<P: IsA<TlsCertificate>>(&self, certificate: &P);

    fn set_database<P: IsA<TlsDatabase>>(&self, database: &P);

    fn set_interaction<P: IsA<TlsInteraction>>(&self, interaction: Option<&P>);

    fn set_rehandshake_mode(&self, mode: TlsRehandshakeMode);

    fn set_require_close_notify(&self, require_close_notify: bool);

    fn get_property_base_io_stream(&self) -> Option<IOStream>;

    fn connect_accept_certificate<F: Fn(&Self, &TlsCertificate, TlsCertificateFlags) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_database_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_interaction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_peer_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_peer_certificate_errors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rehandshake_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_require_close_notify_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TlsConnection>> TlsConnectionExt for O {
    fn emit_accept_certificate<P: IsA<TlsCertificate>>(&self, peer_cert: &P, errors: TlsCertificateFlags) -> bool {
        unsafe {
            from_glib(gio_sys::g_tls_connection_emit_accept_certificate(self.as_ref().to_glib_none().0, peer_cert.as_ref().to_glib_none().0, errors.to_glib()))
        }
    }

    fn get_certificate(&self) -> Option<TlsCertificate> {
        unsafe {
            from_glib_none(gio_sys::g_tls_connection_get_certificate(self.as_ref().to_glib_none().0))
        }
    }

    fn get_database(&self) -> Option<TlsDatabase> {
        unsafe {
            from_glib_none(gio_sys::g_tls_connection_get_database(self.as_ref().to_glib_none().0))
        }
    }

    fn get_interaction(&self) -> Option<TlsInteraction> {
        unsafe {
            from_glib_none(gio_sys::g_tls_connection_get_interaction(self.as_ref().to_glib_none().0))
        }
    }

    fn get_peer_certificate(&self) -> Option<TlsCertificate> {
        unsafe {
            from_glib_none(gio_sys::g_tls_connection_get_peer_certificate(self.as_ref().to_glib_none().0))
        }
    }

    fn get_peer_certificate_errors(&self) -> TlsCertificateFlags {
        unsafe {
            from_glib(gio_sys::g_tls_connection_get_peer_certificate_errors(self.as_ref().to_glib_none().0))
        }
    }

    fn get_rehandshake_mode(&self) -> TlsRehandshakeMode {
        unsafe {
            from_glib(gio_sys::g_tls_connection_get_rehandshake_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn get_require_close_notify(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_tls_connection_get_require_close_notify(self.as_ref().to_glib_none().0))
        }
    }

    fn handshake<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_tls_connection_handshake(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn handshake_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn handshake_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_tls_connection_handshake_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = handshake_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_tls_connection_handshake_async(self.as_ref().to_glib_none().0, io_priority.to_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn handshake_async_future(&self, io_priority: glib::Priority) -> Box_<dyn future::Future<Output = Result<(), Error>> + std::marker::Unpin> {
        use GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.handshake_async(
                io_priority,
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn set_certificate<P: IsA<TlsCertificate>>(&self, certificate: &P) {
        unsafe {
            gio_sys::g_tls_connection_set_certificate(self.as_ref().to_glib_none().0, certificate.as_ref().to_glib_none().0);
        }
    }

    fn set_database<P: IsA<TlsDatabase>>(&self, database: &P) {
        unsafe {
            gio_sys::g_tls_connection_set_database(self.as_ref().to_glib_none().0, database.as_ref().to_glib_none().0);
        }
    }

    fn set_interaction<P: IsA<TlsInteraction>>(&self, interaction: Option<&P>) {
        unsafe {
            gio_sys::g_tls_connection_set_interaction(self.as_ref().to_glib_none().0, interaction.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_rehandshake_mode(&self, mode: TlsRehandshakeMode) {
        unsafe {
            gio_sys::g_tls_connection_set_rehandshake_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }

    fn set_require_close_notify(&self, require_close_notify: bool) {
        unsafe {
            gio_sys::g_tls_connection_set_require_close_notify(self.as_ref().to_glib_none().0, require_close_notify.to_glib());
        }
    }

    fn get_property_base_io_stream(&self) -> Option<IOStream> {
        unsafe {
            let mut value = Value::from_type(<IOStream as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"base-io-stream\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_accept_certificate<F: Fn(&Self, &TlsCertificate, TlsCertificateFlags) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"accept-certificate\0".as_ptr() as *const _,
                Some(transmute(accept_certificate_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::certificate\0".as_ptr() as *const _,
                Some(transmute(notify_certificate_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_database_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::database\0".as_ptr() as *const _,
                Some(transmute(notify_database_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_interaction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::interaction\0".as_ptr() as *const _,
                Some(transmute(notify_interaction_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_peer_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::peer-certificate\0".as_ptr() as *const _,
                Some(transmute(notify_peer_certificate_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_peer_certificate_errors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::peer-certificate-errors\0".as_ptr() as *const _,
                Some(transmute(notify_peer_certificate_errors_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_rehandshake_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rehandshake-mode\0".as_ptr() as *const _,
                Some(transmute(notify_rehandshake_mode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_require_close_notify_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::require-close-notify\0".as_ptr() as *const _,
                Some(transmute(notify_require_close_notify_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn accept_certificate_trampoline<P, F: Fn(&P, &TlsCertificate, TlsCertificateFlags) -> bool + 'static>(this: *mut gio_sys::GTlsConnection, peer_cert: *mut gio_sys::GTlsCertificate, errors: gio_sys::GTlsCertificateFlags, f: glib_sys::gpointer) -> glib_sys::gboolean
where P: IsA<TlsConnection> {
    let f: &F = &*(f as *const F);
    f(&TlsConnection::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(peer_cert), from_glib(errors)).to_glib()
}

unsafe extern "C" fn notify_certificate_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GTlsConnection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<TlsConnection> {
    let f: &F = &*(f as *const F);
    f(&TlsConnection::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_database_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GTlsConnection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<TlsConnection> {
    let f: &F = &*(f as *const F);
    f(&TlsConnection::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_interaction_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GTlsConnection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<TlsConnection> {
    let f: &F = &*(f as *const F);
    f(&TlsConnection::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_peer_certificate_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GTlsConnection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<TlsConnection> {
    let f: &F = &*(f as *const F);
    f(&TlsConnection::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_peer_certificate_errors_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GTlsConnection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<TlsConnection> {
    let f: &F = &*(f as *const F);
    f(&TlsConnection::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rehandshake_mode_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GTlsConnection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<TlsConnection> {
    let f: &F = &*(f as *const F);
    f(&TlsConnection::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_require_close_notify_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GTlsConnection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<TlsConnection> {
    let f: &F = &*(f as *const F);
    f(&TlsConnection::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for TlsConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TlsConnection")
    }
}
