use core::task::{Waker, RawWaker};
use alloc::{sync::Arc, vec::Vec};

use async_lock::{Mutex, MutexGuardArc};

use bytedata::ByteQueue;

struct StreamState<'a> {
    currently_claimed: usize,
    buffer: ByteQueue<'a>,
    waiting_reader: Option<Waker>,
    waiting_writers: Vec<Waker>,
}

pub struct ByteReader<'a> {
    claimed: ByteQueue<'a>,
    state: Arc<Mutex<StreamState<'a>>>,
}

pub struct ByteWriter<'a> {
    sleeping: Option<RawWaker>,
    state: Arc<Mutex<StreamState<'a>>>,
}