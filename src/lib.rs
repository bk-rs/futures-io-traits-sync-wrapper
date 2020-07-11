use std::io::{self, BufRead, Read, Seek, Write};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

pub struct Wrapper<'a, 'b, T> {
    inner: &'a mut T,
    cx: &'a mut Context<'b>,
}

impl<'a, 'b, T> Wrapper<'a, 'b, T> {
    pub fn new(inner: &'a mut T, cx: &'a mut Context<'b>) -> Self {
        Self { inner, cx }
    }

    pub fn get_ref(&self) -> &T {
        self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.inner
    }
}

impl<'a, 'b, T: AsyncWrite + Unpin> Write for Wrapper<'a, 'b, T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match Pin::new(&mut self.inner).poll_write(self.cx, buf) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        match Pin::new(&mut self.inner).poll_flush(self.cx) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }
}

impl<'a, 'b, T: AsyncRead + Unpin> Read for Wrapper<'a, 'b, T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match Pin::new(&mut self.inner).poll_read(self.cx, buf) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }
}

impl<'a, 'b, T: AsyncSeek + Unpin> Seek for Wrapper<'a, 'b, T> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match Pin::new(&mut self.inner).poll_seek(self.cx, pos) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }
}

impl<'a, 'b, T: AsyncBufRead + Unpin> BufRead for Wrapper<'a, 'b, T> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match Pin::new(&mut self.inner).poll_fill_buf(self.cx) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }

    fn consume(&mut self, amt: usize) {
        Pin::new(&mut self.inner).consume(amt)
    }
}
