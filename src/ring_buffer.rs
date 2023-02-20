use std::collections::VecDeque;

pub struct FixedSizeRingBuffer {
    max_elements_count: u8,
    rp: u8,
    wp: u8,
    buffer: VecDeque<u32>,
}
impl FixedSizeRingBuffer {
    pub fn new(size: u8) -> FixedSizeRingBuffer {
        let buf: VecDeque<u32> = VecDeque::new();
        FixedSizeRingBuffer {
            max_elements_count: size,
            rp: 0,
            wp: 0,
            buffer: buf,
        }
    }

    pub fn push(&mut self, value: u32) -> () {
        if self.wp == 0 {
            self.buffer.push_front(value);
            self.wp += 1;
        } else if self.wp > 0 && self.wp < self.max_elements_count {
            self.buffer.push_front(value);
            self.rp += 1;
            self.wp += 1;
        } else {
             self.buffer.pop_back();
             self.buffer.push_front(value);
        }
    }

    pub fn pop(&mut self) -> Option<u32> {
        if self.wp == 0 && self.rp == 0 {
            None
        } else {
            self.wp -= 1;
            if self.wp == 0 {
                self.rp = 0;
            } else {
                self.rp -= 1;
            }
            self.buffer.pop_back()
        }
    }

    pub fn current_status(&self) -> () {
        println!("max elements count: {}", self.max_elements_count);
        println!("read  pointer: {}", self.rp);
        println!("write pointer: {}", self.wp);
        println!("buffer: {:?}", self.buffer);
    }
}

#[test]
fn test_ring() {
    let mut ringbuf = FixedSizeRingBuffer::new(3);
    ringbuf.push(1);
    ringbuf.push(2);

    assert_eq!(ringbuf.pop().unwrap(), 1);
    assert_eq!(ringbuf.pop().unwrap(), 2);
}

#[test]
fn test_ring_full() {
    let mut ringbuf = FixedSizeRingBuffer::new(3);
    ringbuf.push(1);
    ringbuf.push(2);
    ringbuf.push(3);
    ringbuf.push(4);
    ringbuf.push(5);

    assert_eq!(ringbuf.pop().unwrap(), 3);
    assert_eq!(ringbuf.pop().unwrap(), 4);
    assert_eq!(ringbuf.pop().unwrap(), 5);
}

#[test]
fn test_ring_empty() {
    let mut ringbuf = FixedSizeRingBuffer::new(3);
    assert!(ringbuf.pop().is_none());
}
