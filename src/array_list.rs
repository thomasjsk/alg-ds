use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::ptr;

pub struct ArrayList<T> {
    array: *mut T,
    len: usize,
    cap: usize,
}

impl<T> ArrayList<T>
where
    T: Clone + Default,
{
    pub fn new(cap: usize) -> ArrayList<T> {
        if cap == 0 {
            return ArrayList {
                array: ArrayList::with_capacity(0),
                len: 0,
                cap: 0,
            };
        }

        ArrayList {
            array: ArrayList::with_capacity(cap),
            len: 0,
            cap,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn print(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.array, self.cap) }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.resize(self.cap * 2);
        }

        unsafe {
            ptr::write(self.array.add(self.len), value);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        let item = unsafe { Some(ptr::read(self.array.add(self.len))) };
        unsafe {
            ptr::write(self.array.add(self.len), T::default()); // This might not be needed as we operate on len anyway
        }

        item
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let item = unsafe { Some(ptr::read(self.array.add(0))) };
        for i in 0..self.len {
            unsafe {
                ptr::write(self.array.add(i), ptr::read(self.array.add(i + 1)));
            }
        }

        self.len -= 1;
        self.cap -= 1;

        unsafe {
            ptr::drop_in_place(self.array.add(self.len));
        }

        item
    }

    pub fn push_front(&mut self, value: T) {
        if self.len == self.cap {
            self.resize(self.cap * 2);
        }

        for i in (0..self.len).rev() {
            unsafe { ptr::write(self.array.add(i + 1), ptr::read(self.array.add(i))) };
        }

        unsafe { ptr::write(self.array.add(0), value) };

        self.len += 1;
    }

    fn with_capacity(cap: usize) -> *mut T {
        if cap == 0 {
            return ptr::null_mut();
        }

        let layout = Layout::array::<T>(cap).unwrap();
        let array = unsafe { alloc(layout) as *mut T };
        if array.is_null() {
            handle_alloc_error(layout);
        }

        for i in 0..cap {
            unsafe {
                ptr::write(array.add(i), T::default());
            }
        }

        array
    }

    fn resize(&mut self, cap: usize) {
        let new_array = Self::with_capacity(cap);
        unsafe {
            ptr::copy(self.array, new_array, self.cap);
        }

        if self.cap > 0 {
            unsafe {
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                dealloc(self.array as *mut u8, old_layout);
            }
        }

        self.array = new_array;
        self.cap = cap;
    }
}

#[cfg(test)]
mod tests {
    use crate::array_list::ArrayList;

    #[test]
    fn growing() {
        let initial_cap = 2;
        let mut list = ArrayList::<u32>::new(initial_cap);
        assert_eq!(list.len(), 0);
        assert_eq!(list.print(), &[0, 0]);
        assert_eq!(list.cap(), initial_cap);

        list.push(1);
        assert_eq!(list.print(), &[1, 0]);
        assert_eq!(list.len(), 1);
        assert_eq!(list.cap(), initial_cap);

        list.push(2);
        assert_eq!(list.print(), &[1, 2]);
        assert_eq!(list.cap(), initial_cap);
        let mut prev_cap = list.cap();

        list.push(3);
        assert_eq!(list.print(), &[1, 2, 3, 0]);
        assert_eq!(list.cap(), prev_cap * 2);

        list.push(4);
        assert_eq!(list.print(), &[1, 2, 3, 4]);
        assert_eq!(list.cap(), prev_cap * 2);
        prev_cap = list.cap();

        list.push(5);
        assert_eq!(list.print(), &[1, 2, 3, 4, 5, 0, 0, 0]);
        assert_eq!(list.cap(), prev_cap * 2);
    }

    #[test]
    fn pop() {
        let initial_cap = 4;
        let mut list = ArrayList::<u32>::new(initial_cap);
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.print(), &[1, 2, 3, 0]);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.print(), &[1, 2, 0, 0]);

        list.push(4);
        list.push(5);
        assert_eq!(list.print(), &[1, 2, 4, 5]);

        list.push(6);
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.print(), &[1, 2, 4, 5, 0, 0, 0, 0]);
    }

    #[test]
    fn pop_front() {
        let initial_cap = 3;
        let mut list = ArrayList::<u32>::new(initial_cap);
        assert_eq!(list.pop_front(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.print(), &[2, 3]);

        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.print(), &[3]);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.print(), &[]);

        assert_eq!(list.pop_front(), None);
        assert_eq!(list.print(), &[]);
    }

    #[test]
    fn push_front() {
        let initial_cap = 3;
        let mut list = ArrayList::<u32>::new(initial_cap);
        assert_eq!(list.len(), 0);

        list.push_front(1);
        assert_eq!(list.print(), &[1, 0, 0]);

        list.push_front(2);
        assert_eq!(list.print(), &[2, 1, 0]);

        list.push_front(3);
        assert_eq!(list.print(), &[3, 2, 1]);

        list.push_front(4);
        assert_eq!(list.print(), &[4, 3, 2, 1, 0, 0]);
    }

    #[test]
    fn push_front_and_back() {
        let initial_cap = 4;
        let mut list = ArrayList::<u32>::new(initial_cap);

        list.push_front(1);
        assert_eq!(list.print(), &[1, 0, 0, 0]);

        list.push(2);
        assert_eq!(list.print(), &[1, 2, 0, 0]);

        list.push_front(3);
        assert_eq!(list.print(), &[3, 1, 2, 0]);

        list.push(4);
        assert_eq!(list.print(), &[3, 1, 2, 4]);
    }
}
