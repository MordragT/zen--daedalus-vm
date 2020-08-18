use zen_memory::{Allocator, Handle};

pub struct ObjectAllocator<T: Default> {
    allocator: Allocator<T>,
    current: usize,
    limit: usize,
}

impl<T: Default> ObjectAllocator<T> {
    pub fn new(limit: usize) -> Self {
        let allocator = Allocator::<T>::new();
        Self {
            allocator,
            current: 0,
            limit,
        }
    }
    fn left_space(&self) -> usize {
        self.limit - self.current
    }
    pub fn create(&mut self) -> Result<Handle, &str> {
        match self.left_space() > 0 {
            true => {
                self.current += 1;
                self.allocator.create()
            }
            false => Err("No space left"),
        }
    }
    pub fn remove(&mut self, handle: &Handle) {
        self.current -= 1;
        self.allocator.remove(handle);
    }
    pub fn get(&self, handle: &Handle) -> Result<&T, &str> {
        match self.allocator.get(handle) {
            Some(val) => Ok(val),
            None => Err("Could not get value, pointed to from the handle"),
        }
    }
    pub fn get_mut(&self, handle: &Handle) -> Result<&mut T, &str> {
        match self.allocator.get_mut(handle) {
            Some(val) => Ok(val),
            None => Err("Could not get value, pointed to from the handle"),
        }
    }
}
