//! Deprecated, Please use [async-stream-packed](https://docs.rs/async-stream-packed) crate.

use async_stream_packed::SyncableWithContextAsyncStream;

#[deprecated(note = "Please use `async-stream-packed` crate")]
pub type Wrapper<'a, 'b, T> = SyncableWithContextAsyncStream<'a, 'b, T>;
