// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Cancellable;
use Error;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib::Priority;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;
use OutputStream;

#[cfg(feature = "futures")]
use futures_core::Future;

pub trait OutputStreamExtManual: Sized {
    fn write_async<'a, B: AsRef<[u8]> + Send + 'static, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(B, usize), (B, Error)>) + Send + 'static>(&self, buffer: B, io_priority: Priority, cancellable: P, callback: Q);

    fn write_all<'a, P: Into<Option<&'a Cancellable>>>(&self, buffer: &[u8], cancellable: P) -> Result<(usize, Option<Error>), Error>;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn write_all_async<'a, B: AsRef<[u8]> + Send + 'static, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(B, usize, Option<Error>), (B, Error)>) + Send + 'static>(&self, buffer: B, io_priority: Priority, cancellable: P, callback: Q);

    #[cfg(feature = "futures")]
    fn write_async_future<'a, B: AsRef<[u8]> + Send + 'static>(
        &self, buffer: B, io_priority: Priority
    ) -> Box<Future<Item = (Self, (B, usize)), Error = (Self, (B, Error))>>;

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn write_all_async_future<'a, B: AsRef<[u8]> + Send + 'static>(
        &self, buffer: B, io_priority: Priority
    ) -> Box<Future<Item = (Self, (B, usize, Option<Error>)), Error = (Self, (B, Error))>>;
}

impl<O: IsA<OutputStream> + IsA<glib::Object> + Clone + 'static> OutputStreamExtManual for O {
    fn write_async<'a, B: AsRef<[u8]> + Send + 'static, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(B, usize), (B, Error)>) + Send + 'static>(&self, buffer: B, io_priority: Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let buffer: Box<B> = Box::new(buffer);
        let (count, buffer_ptr) = {
            let slice = (*buffer).as_ref();
            (slice.len(), slice.as_ptr())
        };
        let user_data: Box<Option<(Box<Q>, Box<B>)>> = Box::new(Some((Box::new(callback), buffer)));
        unsafe extern "C" fn write_async_trampoline<B: AsRef<[u8]> + Send + 'static, Q:FnOnce(Result<(B, usize), (B, Error)>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut user_data: Box<Option<(Box<Q>, Box<B>)>> = Box::from_raw(user_data as *mut _);
            let (callback, buffer) = user_data.take().unwrap();
            let buffer = *buffer;

            let mut error = ptr::null_mut();
            let ret = ffi::g_output_stream_write_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok((buffer, ret as usize))
            } else {
                Err((buffer, from_glib_full(error)))
            };
            callback(result);
        }
        let callback = write_async_trampoline::<B, Q>;
        unsafe {
            ffi::g_output_stream_write_async(self.to_glib_none().0, mut_override(buffer_ptr), count, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn write_all<'a, P: Into<Option<&'a Cancellable>>>(&self, buffer: &[u8], cancellable: P) -> Result<(usize, Option<Error>), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let count = buffer.len() as usize;
        unsafe {
            let mut bytes_written = mem::uninitialized();
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_write_all(self.to_glib_none().0, buffer.to_glib_none().0, count, &mut bytes_written, cancellable.0, &mut error);

            if error.is_null() {
                Ok((bytes_written, None))
            } else if bytes_written != 0 {
                Ok((bytes_written, Some(from_glib_full(error))))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn write_all_async<'a, B: AsRef<[u8]> + Send + 'static, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(B, usize, Option<Error>), (B, Error)>) + Send + 'static>(&self, buffer: B, io_priority: Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let buffer: Box<B> = Box::new(buffer);
        let (count, buffer_ptr) = {
            let slice = (*buffer).as_ref();
            (slice.len(), slice.as_ptr())
        };
        let user_data: Box<Option<(Box<Q>, Box<B>)>> = Box::new(Some((Box::new(callback), buffer)));
        unsafe extern "C" fn write_all_async_trampoline<B: AsRef<[u8]> + Send + 'static, Q: FnOnce(Result<(B, usize, Option<Error>), (B, Error)>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut user_data: Box<Option<(Box<Q>, Box<B>)>> = Box::from_raw(user_data as *mut _);
            let (callback, buffer) = user_data.take().unwrap();
            let buffer = *buffer;

            let mut error = ptr::null_mut();
            let mut bytes_written = mem::uninitialized();
            let _ = ffi::g_output_stream_write_all_finish(_source_object as *mut _, res, &mut bytes_written, &mut error);
            let result = if error.is_null() {
                Ok((buffer, bytes_written, None))
            } else if bytes_written != 0 {
                Ok((buffer, bytes_written, from_glib_full(error)))
            } else {
                Err((buffer, from_glib_full(error)))
            };
            callback(result);
        }
        let callback = write_all_async_trampoline::<B, Q>;
        unsafe {
            ffi::g_output_stream_write_all_async(self.to_glib_none().0, mut_override(buffer_ptr), count, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }


    #[cfg(feature = "futures")]
    fn write_async_future<'a, B: AsRef<[u8]> + Send + 'static>(
        &self, buffer: B, io_priority: Priority
    ) -> Box<Future<Item = (Self, (B, usize)), Error = (Self, (B, Error))>> {
        use GioFuture;

        GioFuture::new(self, move |obj, send| {
            use fragile::Fragile;

            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.write_async(
                buffer,
                io_priority,
                Some(&cancellable),
                move |res| {
                    let obj = obj_clone.into_inner();
                    let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn write_all_async_future<'a, B: AsRef<[u8]> + Send + 'static>(
        &self, buffer: B, io_priority: Priority
    ) -> Box<Future<Item = (Self, (B, usize, Option<Error>)), Error = (Self, (B, Error))>> {
        use GioFuture;

        GioFuture::new(self, move |obj, send| {
            use fragile::Fragile;

            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.write_all_async(
                buffer,
                io_priority,
                Some(&cancellable),
                move |res| {
                    let obj = obj_clone.into_inner();
                    let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }
}

#[cfg(all(test,any(feature = "v2_36", feature = "dox")))]
mod tests {
    use glib::*;
    use test_util::run_async;
    use *;

    #[test]
    fn splice_async() {
        let ret = run_async(|tx, l| {
            let input = MemoryInputStream::new();
            let b = Bytes::from_owned(vec![1, 2, 3]);
            input.add_bytes(&b);

            let strm = MemoryOutputStream::new_resizable();
            strm.splice_async(&input, OutputStreamSpliceFlags::CLOSE_SOURCE,
                              PRIORITY_DEFAULT_IDLE, None, move |ret| {
                tx.send(ret).unwrap();
                l.quit();
            });
        });

        assert_eq!(ret.unwrap(), 3);
    }

    #[test]
    fn write_async() {
        let ret = run_async(|tx, l| {
            let strm = MemoryOutputStream::new_resizable();

            let buf = vec![1,2,3];
            strm.write_async(buf, PRIORITY_DEFAULT_IDLE, None, move |ret| {
                tx.send(ret).unwrap();
                l.quit();
            });
        });

        let (buf, size) = ret.unwrap();
        assert_eq!(buf, vec![1,2,3]);
        assert_eq!(size, 3);
    }

    #[test]
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn write_all_async() {
        let ret = run_async(|tx, l| {
            let strm = MemoryOutputStream::new_resizable();

            let buf = vec![1,2,3];
            strm.write_all_async(buf, PRIORITY_DEFAULT_IDLE, None, move |ret| {
                tx.send(ret).unwrap();
                l.quit();
            });
        });

        let (buf, size, err) = ret.unwrap();
        assert_eq!(buf, vec![1,2,3]);
        assert_eq!(size, 3);
        assert!(err.is_none());
    }

    #[test]
    fn write_bytes_async() {
        let ret = run_async(|tx, l| {
            let strm = MemoryOutputStream::new_resizable();

            let b = Bytes::from_owned(vec![1, 2, 3]);
            strm.write_bytes_async(&b, PRIORITY_DEFAULT_IDLE, None, move |ret| {
                tx.send(ret).unwrap();
                l.quit();
            });
        });

        assert_eq!(ret.unwrap(), 3);
    }
}
