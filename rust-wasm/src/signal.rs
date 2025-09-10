use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use wasm_bindgen::prelude::wasm_bindgen;

pub(crate) struct DropDetector<T, F>
where
    T: Copy,
    F: Fn(T) -> (),
{
    pub(crate) s: T,
    pub(crate) f: F,
}

impl<T, F> Drop for DropDetector<T, F>
where
    T: Copy,
    F: Fn(T) -> (),
{
    fn drop(&mut self) {
        (self.f)(self.s)
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct AbortSignal {
    state: Arc<Mutex<AbortState>>,
}

struct AbortState {
    aborted: bool,
    waker: Option<Waker>,
}

impl Future for AbortSignal {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        if state.aborted {
            return Poll::Ready(());
        }
        state.waker = Some(cx.waker().clone()).into();
        Poll::Pending
    }
}

#[wasm_bindgen]
impl AbortSignal {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AbortSignal {
        AbortSignal {
            state: Arc::new(Mutex::new(AbortState {
                aborted: false,
                waker: None.into(),
            })),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn aborted(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.aborted
    }

    #[wasm_bindgen]
    pub fn abort(&self) {
        let mut state = self.state.lock().unwrap();
        state.aborted = true;
        state.waker.take().map(|waker| waker.wake());
    }
}
