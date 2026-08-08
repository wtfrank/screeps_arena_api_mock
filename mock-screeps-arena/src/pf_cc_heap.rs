// Direct translation of Marcel Laverdet's heap_t and open_closed_t from Screeps C++ pf.h

pub struct PfCcHeap {
    priorities: [u32; 10000],
    heap: [u16; 10000],
    size: usize,
}

impl PfCcHeap {
    pub fn new() -> Self {
        Self {
            priorities: [0; 10000],
            heap: [0; 10000],
            size: 0,
        }
    }

    #[inline]
    pub fn empty(&self) -> bool {
        self.size == 0
    }

    #[inline]
    pub fn priority(&self, index: usize) -> u32 {
        self.priorities[index]
    }

    pub fn pop(&mut self) -> (usize, u32) {
        let idx = self.heap[1] as usize;
        let prio = self.priorities[idx];
        let ret = (idx, prio);
        self.heap[1] = self.heap[self.size];
        self.size -= 1;
        let mut vv = 1;
        loop {
            let uu = vv;
            let left = uu << 1;
            let right = left + 1;
            if right <= self.size {
                if self.priorities[self.heap[uu] as usize] >= self.priorities[self.heap[left] as usize] {
                    vv = left;
                }
                if self.priorities[self.heap[vv] as usize] >= self.priorities[self.heap[right] as usize] {
                    vv = right;
                }
            } else if left <= self.size {
                if self.priorities[self.heap[uu] as usize] >= self.priorities[self.heap[left] as usize] {
                    vv = left;
                }
            }
            if uu != vv {
                self.heap.swap(uu, vv);
            } else {
                break;
            }
        }
        ret
    }

    pub fn insert(&mut self, index: usize, priority: u32) {
        self.priorities[index] = priority;
        self.size += 1;
        self.heap[self.size] = index as u16;
        self.bubble_up(self.size);
    }

    pub fn update(&mut self, index: usize, priority: u32) {
        let mut ii = self.size;
        while ii > 0 {
            if self.heap[ii] as usize == index {
                self.priorities[index] = priority;
                self.bubble_up(ii);
                return;
            }
            ii -= 1;
        }
    }

    fn bubble_up(&mut self, mut ii: usize) {
        while ii != 1 {
            let parent = ii >> 1;
            if self.priorities[self.heap[ii] as usize] <= self.priorities[self.heap[parent] as usize] {
                self.heap.swap(ii, parent);
                ii = parent;
            } else {
                return;
            }
        }
    }

    pub fn clear(&mut self) {
        self.size = 0;
    }
}

pub struct OpenClosedList {
    list: [u32; 10000],
    marker: u32,
}

impl OpenClosedList {
    pub fn new() -> Self {
        Self {
            list: [0; 10000],
            marker: 1,
        }
    }

    pub fn clear(&mut self) {
        if u32::MAX - 2 <= self.marker {
            self.list.fill(0);
            self.marker = 1;
        } else {
            self.marker += 2;
        }
    }

    #[inline]
    pub fn is_open(&self, index: usize) -> bool {
        self.list[index] == self.marker
    }

    #[inline]
    pub fn is_closed(&self, index: usize) -> bool {
        self.list[index] == self.marker + 1
    }

    #[inline]
    pub fn open(&mut self, index: usize) {
        self.list[index] = self.marker;
    }

    #[inline]
    pub fn close(&mut self, index: usize) {
        self.list[index] = self.marker + 1;
    }
}
