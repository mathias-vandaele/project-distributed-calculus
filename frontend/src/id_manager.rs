pub struct IDManager {
    id : u128
}

impl IDManager {
    pub fn new() -> IDManager {
        IDManager {
            id : 0
        }
    }

    pub fn next(&mut self) -> u128 {
        self.id += 1;
        self.id
    }
}