use macroquad::audio::{load_sound_from_bytes, Sound};
use macroquad::prelude::{load_image, load_texture, FileError, Image, Texture2D};
use std::future::Future;
use std::pin::Pin;
use std::task::{Poll, RawWaker, RawWakerVTable, Waker};
use crate::AnyError;

/// Loads resources semi-asynchronously, so that you can render a loading screen.
///
/// This is not fully asynchronous because once the resource is loaded, there may be a format
/// conversion that will be blocking. Still, using this struct is an improvement compared to
/// blocking during both the load and the format conversion.
///
/// I have tested that this struct works as expected in linux and wasm. (Browsers were particularly
/// prone to suffer from the blocking during load).
///
/// See [`examples/hello_juquad.rs:36`] for an example of how to do a loading screen while waiting
/// for this to load.
pub struct ResourceLoader<'a, T, E, F, Fut>
where
    E: Into<AnyError>,
    F: Fn(&'a [u8]) -> Fut,
    Fut: Future<Output = Result<T, E>> + 'a,
{
    resources_bytes: &'a [&'a [u8]],
    resources: Vec<T>,
    in_progress: Option<Pin<Box<Fut>>>,
    load_func: F,
}

pub struct Progress {
    pub loaded: usize,
    pub total_to_load: usize,
}

impl<'a, T, E, F, Fut> ResourceLoader<'a, T, E, F, Fut>
where
    E: Into<AnyError>,
    F: Fn(&'a [u8]) -> Fut,
    Fut: Future<Output = Result<T, E>> + 'a,
{
    pub fn new(resources_bytes: &'a [&'a [u8]], load_func: F) -> Self {
        Self {
            resources_bytes,
            resources: Vec::new(),
            in_progress: None,
            load_func,
        }
    }

    pub fn get_progress(&self) -> Progress {
        Progress {
            loaded: self.resources.len(),
            total_to_load: self.resources_bytes.len(),
        }
    }

    /// Ok(None) until all are loaded; Ok(Some(vec)) when done; Err on load error.
    pub fn get_resources(&mut self) -> Result<Option<Vec<T>>, E> {
        if self.resources.len() < self.resources_bytes.len() {
            let i = self.resources.len();

            if let Some(in_progress) = &mut self.in_progress {
                // Pin<Box<Fut>> -> Pin<&mut Fut>
                if let Some(res) = resume(in_progress.as_mut()) {
                    let resource = res?;
                    self.resources.push(resource);
                    self.in_progress = None;
                }
            } else {
                // start next load
                let fut = (self.load_func)(self.resources_bytes[i]);
                self.in_progress = Some(Box::pin(fut));
            }

            Ok(None)
        } else {
            let mut out = Vec::new();
            std::mem::swap(&mut out, &mut self.resources);
            Ok(Some(out))
        }
    }
}

// resume() and waker() taken from macroquad::exec. I don't understand why they are private
// I only made them generic over Fut

/// returns Some(T) if future is done, None if it would block
fn resume<Fut>(mut future: Pin<&mut Fut>) -> Option<Fut::Output>
where
    Fut: Future,
{
    let waker = waker();
    let mut futures_context = std::task::Context::from_waker(&waker);
    match Future::poll(future.as_mut(), &mut futures_context) {
        Poll::Ready(v) => Some(v),
        Poll::Pending => None,
    }
}
fn waker() -> Waker {
    unsafe fn clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &VTABLE)
    }
    unsafe fn wake(_data: *const ()) {
        panic!(
            "macroquad does not support waking futures, please use coroutines, \
            otherwise your pending future will block until the next frame"
        )
    }
    unsafe fn wake_by_ref(data: *const ()) {
        wake(data)
    }
    unsafe fn drop(_data: *const ()) {
        // Nothing to do
    }
    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    let raw_waker = RawWaker::new(std::ptr::null(), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}
