//! A small RAII wrapper around a [`Stream`] that automatically invokes a
//! user-defined action upon being dropped.
//! 
//! For example:
//! 
//! ```rust
//! # use futures::stream::{self, StreamExt};
//! # use stream_guard::GuardStreamExt;
//! #
//! async fn f() {
//!     let mut s = stream::iter(0..3).guard(|| println!("Dropped!"));
//!     while let Some(i) = s.next().await {
//!         println!("{}", i);
//!     }
//! }
//! ```
//! 
//! would print
//! 
//! ```plaintext
//! 0
//! 1
//! 2
//! Dropped!
//! ```

use std::{pin::Pin, task::{Context, Poll}};

use futures::Stream;
use pin_project::{pin_project, pinned_drop};

/// A [`Stream`] wrapper that automatically runs a custom action when dropped.
#[pin_project(PinnedDrop)]
pub struct StreamGuard<S, F> where S: Stream, F: FnOnce() {
    #[pin]
    stream: S,
    on_drop: Option<F>,
}

impl<S, F> StreamGuard<S, F> where S: Stream, F: FnOnce() {
    /// Wraps the given [`Stream`], running the given closure upon being dropped.
    pub fn new(stream: S, on_drop: F) -> Self {
        Self { stream, on_drop: Some(on_drop) }
    }
}

impl<S, F> Stream for StreamGuard<S, F> where S: Stream, F: FnOnce() {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}

#[pinned_drop]
impl<S, F> PinnedDrop for StreamGuard<S, F> where S: Stream, F: FnOnce() {
    fn drop(mut self: Pin<&mut Self>) {
        self.project().on_drop.take().expect("No on_drop function in StreamGuard, was drop called twice or constructed wrongly?")()
    }
}

/// A convenience extension for creating a [`StreamGuard`] via a method.
pub trait GuardStreamExt: Stream + Sized {
    /// Wraps the [`Stream`], running the given closure upon being dropped.
    fn guard<F>(self, on_drop: F) -> StreamGuard<Self, F> where F: FnOnce();
}

impl<S> GuardStreamExt for S where S: Stream + Sized {
    fn guard<F>(self, on_drop: F) -> StreamGuard<Self, F> where F: FnOnce() {
        StreamGuard::new(self, on_drop)
    }
}
