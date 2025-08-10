use rdev::{Key};
pub struct KeyCombination {
    map: std::collections::HashMap<Key, bool>,
    is_active: bool,
    press_count: usize,
}

impl KeyCombination {
    pub fn new(first: Key, second: Key) -> Self {
        let mut map = std::collections::HashMap::new();
        map.insert(first, false);
        map.insert(second, false);
        KeyCombination { 
            map, 
            is_active: false,
            press_count: 0
        }
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    pub fn contains(&self, key: Key) -> bool {
        self.map.contains_key(&key)
    }
    pub fn press_key(&mut self, key: Key) {
        if let Some(pressed) = self.map.get_mut(&key) {
            if *pressed != true {
                self.press_count += 1;
            }
            *pressed = true;
            if self.press_count == self.map.len() {
                self.is_active = true;
            }
        }
    }
    pub fn release_key(&mut self, key: Key) {
        if let Some(pressed) = self.map.get_mut(&key) {
            *pressed = false;
            if self.press_count > 0 {
                self.press_count -= 1;
            }
            if self.press_count < self.map.len() {
                self.is_active = false;
            }
        }
    }

}

