use std::collections::HashMap;
use std::fmt;

pub struct Field(pub String, pub String);

#[derive(Debug)]
pub struct RecordStore {
    records: HashMap<String, HashMap<String, String>>
}

impl RecordStore {
    pub fn new() -> RecordStore {
        RecordStore {
            records: HashMap::new()
        }
    }

    pub fn add_empty(&mut self, name: String) {
        self.records.insert(name, HashMap::new());
    }

    pub fn add_field(&mut self, record_key: &str, field: Field) {
        if let Some(fields) = self.records.get_mut(record_key) {
            let Field(name, val) = field;
            fields.insert(name, val);
        }
    }
}