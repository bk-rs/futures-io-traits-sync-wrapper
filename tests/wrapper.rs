use std::io::{self, BufRead, Read, Seek, Write};
use std::task::Poll;

use futures_executor::block_on;
use futures_util::future;
use futures_util::io::Cursor;

use futures_io_traits_sync_wrapper::Wrapper;

#[test]
fn cursor() -> io::Result<()> {
    block_on(async {
        let mut cursor = Cursor::new(Vec::<u8>::new());

        // test Write
        cursor.set_position(0);
        future::poll_fn(|cx| {
            let mut wrapper = Wrapper::new(&mut cursor, cx);

            assert_eq!(wrapper.write(b"foo").ok(), Some(3));
            assert!(wrapper.flush().is_ok());

            Poll::Ready(())
        })
        .await;
        assert_eq!(cursor.get_ref(), b"foo");

        // test BufRead and Seek
        cursor.set_position(0);
        future::poll_fn(|cx| {
            let mut wrapper = Wrapper::new(&mut cursor, cx);

            assert_eq!(wrapper.fill_buf().ok(), Some(&b"foo"[..]));

            wrapper.consume(1);
            assert_eq!(wrapper.fill_buf().ok(), Some(&b"oo"[..]));

            wrapper.consume(0);

            assert_eq!(wrapper.seek(io::SeekFrom::Start(2)).ok(), Some(2));
            assert_eq!(wrapper.fill_buf().ok(), Some(&b"o"[..]));

            Poll::Ready(())
        })
        .await;
        assert_eq!(cursor.get_ref(), b"foo");

        // test Read
        cursor.set_position(0);
        future::poll_fn(|cx| {
            let mut wrapper = Wrapper::new(&mut cursor, cx);

            let mut buf = vec![0; 5];
            assert_eq!(wrapper.read(&mut buf).ok(), Some(3));

            assert_eq!(buf, b"foo\0\0");

            Poll::Ready(())
        })
        .await;
        assert_eq!(cursor.get_ref(), b"foo");

        Ok(())
    })
}
