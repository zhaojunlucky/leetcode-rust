use std::collections::{HashMap};
use rand::Rng;

struct RandomizedSet {
    arr: Vec<i32>,
    map: HashMap<i32, i32>,
    index: usize
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        Self {
            arr: vec![],
            map: HashMap::new(),
            index: 0,
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false
        }
        self.arr.insert(self.index, val);

        self.map.insert(val, self.index as i32);
        self.index += 1;
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            return false
        }

        let index = *self.map.get(&val).unwrap() as usize;
        self.map.insert(self.arr[self.index - 1], index as i32);
        self.arr.swap(index, self.index - 1);
        self.arr.pop();
        self.map.remove(&val);
        self.index -= 1;
        true

    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::rng();
        let random_index = rng.random_range(0..self.index);

        self.arr[random_index]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_randomized_set() {
        let mut randomized_set = RandomizedSet::new();
        assert!(randomized_set.insert(0));
        assert!(randomized_set.insert(1));
        assert!(randomized_set.remove(0));
        assert!(randomized_set.insert(2));
        assert!(randomized_set.remove(1));
        assert_eq!(2, randomized_set.get_random());
    }
}