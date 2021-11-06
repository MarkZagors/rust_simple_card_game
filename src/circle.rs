use std::collections::LinkedList;

pub struct CircleList<T> {
    pub data: LinkedList<T>,
    pub index: usize,
}

impl<T> CircleList<T> {
    pub fn new() -> CircleList<T> {
        CircleList {
            data: LinkedList::new(),
            index: 0,
        }
    }

    pub fn move_next(&mut self) {
        self.index += 1;
        if self.index > self.data.len()-1 {
            self.index = 0;
        }
    }

    pub fn move_prev(&mut self) {
        if self.index == 0 { self.index = self.data.len()-1; }
        else { self.index -= 1; }
    }

    pub fn get_current(&self) -> &T {
        self.data.iter().nth(self.index).unwrap()
    }
}