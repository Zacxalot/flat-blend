use std::mem::ManuallyDrop;

use super::bm_loop::BMLoop;

pub struct BMFace {
    loop_start: *mut ManuallyDrop<BMLoop>,
    loop_len: usize,
}
