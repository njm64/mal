use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::types::*;

#[derive(Clone)]
pub struct MalEnv {
    data : Rc<RefCell<MalEnvData>>
}

struct MalEnvData {
    outer : Option<MalEnv>,
    map : HashMap<String, MalValue>
}

impl MalEnv {

    pub fn new(outer: Option<MalEnv>) -> MalEnv {
        let data = MalEnvData {
            outer : outer,
            map : HashMap::new()
        };

        MalEnv {data : Rc::new(RefCell::new(data))}
    }


    pub fn find(self: &MalEnv, key : &str) -> Option<MalEnv> {
        let data = self.data.borrow();

        if data.map.contains_key(key) {
            return Some(self.clone());
        }

        if let Some(outer) = &data.outer {
            return outer.find(key);
        }

        return None;
    }

    pub fn set(self: &MalEnv, key : &str, value : MalValue) {
        self.data.borrow_mut().map.insert(key.to_string(), value);
    }

    pub fn get(self: &MalEnv, key : &str) -> Option<MalValue> {
        let data = self.data.borrow();

        if let Some(value) = data.map.get(key) {
            return Some(value.clone());
        }

        if let Some(outer) = &data.outer {
            return outer.get(key)
        }

        return None;
    }

}

