pub struct Counter {
    id : u128
}

impl Counter {
    pub fn new(start : u128) -> Counter {
        Counter {
            id : start
        }
    }

    pub fn next(&mut self) -> u128 {
        self.id += 1;
        self.id
    }
}