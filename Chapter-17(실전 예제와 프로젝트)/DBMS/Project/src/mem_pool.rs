use std::ptr::NonNull;

pub struct MemPool<T> {
    block_size: usize,
    chunk_size: usize,
    free_list: Vec<NonNull<T>>,
    chunks: Vec<Box<[u8]>>,
    active_count: usize,
}

impl<T> MemPool<T> {
    pub fn new(block_size: usize, chunk_size: usize) -> Self {
        assert!(chunk_size >= 1024);
        assert!(block_size >= std::mem::size_of::<usize>());
        MemPool {
            block_size,
            chunk_size,
            free_list: Vec::new(),
            chunks: Vec::new(),
            active_count: 0,
        }
    }

    pub fn alloc(&mut self) -> NonNull<T> {
        if self.free_list.is_empty() {
            self.add_chunk();
        }
        self.active_count += 1;
        self.free_list.pop().unwrap()
    }

    pub fn dealloc(&mut self, ptr: NonNull<T>) {
        self.free_list.push(ptr);
        self.active_count -= 1;
        if self.active_count == 0 {
            self.clear();
        }
    }

    fn add_chunk(&mut self) {
        let count = (self.chunk_size - std::mem::size_of::<usize>()) / self.block_size;
        let mut chunk = vec![0u8; self.chunk_size].into_boxed_slice();
        let base = chunk.as_mut_ptr();

        for i in 0..count {
            let ptr = unsafe { base.add(i * self.block_size) as *mut T };
            self.free_list.push(NonNull::new(ptr).unwrap());
        }

        self.chunks.push(chunk);
    }

    pub fn clear(&mut self) {
        self.free_list.clear();
        self.chunks.clear();
        self.active_count = 0;
    }
}