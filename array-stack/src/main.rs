#[derive(Clone)]
pub struct List<T> {
    pub buf: Vec<T>,
    pub length: usize,
    pub cap: usize,
}

trait ArrayStack<T> {
    fn new(len: usize) -> List<T>;
    fn size(self) -> usize;
    fn get(&self, idx: usize) -> &T;
    fn set(&mut self, idx: usize, val: T);
    fn add(&mut self, idx: usize, val: T);
    fn remove(&mut self, idx: usize) -> &T;
    fn resize(&mut self);
}

impl<T> ArrayStack<T> for List<T> {
    fn new(len: usize) -> List<T> {
        return List {
            buf: Vec::with_capacity(len),
            length: len,
            cap: len,
        };
    }

    fn size(self) -> usize {
        self.length
    }

    fn get(&self, idx: usize) -> &T {
        if idx >= self.cap {
            panic!("get: Array out of range");
        }
        &self.buf[idx]
    }

    fn set(&mut self, idx: usize, val: T) {
        if idx >= self.cap {
            panic!("get: Array out of range");
        }
        self.buf.push(val);
    }

    fn add(&mut self, idx: usize, val: T) {
        if self.length + 1 >= self.cap {
            self.resize();
        }
        for j in self.length..idx {
            self.buf.swap(j, j - 1);
        }
        self.buf[idx] = val;
        self.length += 1;
    }

    fn remove(&mut self, idx: usize) -> &T {
        let tmp: &T = &self.buf[idx];
        for j in idx..self.length {
            self.buf[j] = self.buf[j + 1];
        }
        self.length -= 1;
        if self.cap >= 3 * self.length {
            self.resize();
        }
        return &tmp;
    }

    fn resize(&mut self) {
        self.buf.resize(self.length * 2, 0);
    }
}

fn main() {}

#[test]
fn it_creates_new_instance_with_fixed_size() {
    let a = List::<i32>::new(3);
    assert_eq!(a.size(), 3);
}

#[test]
fn it_gets_correct_element_in_list() {
    let mut a = List::<i32>::new(3);
    a.set(0, 0);
    a.set(1, 1);
    a.set(2, 2);
    assert_eq!(*a.get(0), 0);
    assert_eq!(*a.get(1), 1);
    assert_eq!(*a.get(2), 2);
}

#[test]
fn it_adds_new_elements_in_list() {
    let mut a = List::<i32>::new(3);
    a.add(0, 0);
    a.add(1, 1);
    a.add(2, 2);
    assert_eq!(*a.get(0), 0);
    assert_eq!(*a.get(1), 1);
    assert_eq!(*a.get(2), 2);
}
