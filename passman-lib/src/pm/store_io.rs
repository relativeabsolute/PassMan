extern crate itertools;

use crate::pm::recordstore::RecordStore;
use itertools::Itertools;

pub trait RecordStoreWriter {
    fn new() -> Self;

    fn get_output(&self) -> &str;

    fn write(&mut self, records: &RecordStore);
}

pub struct FileWriter {
    file_name: String
}

impl FileWriter {
    fn set_file_name(&mut self, new_file_name : String) {
        self.file_name = new_file_name;
    }

    fn get_file_name(&self) -> &str {
        return &self.file_name;
    }
}

impl RecordStoreWriter for FileWriter {
    fn new() -> FileWriter {
        FileWriter { file_name: String::from("") }
    }

    // TODO: fill this in 
    fn get_output(&self) -> &str {
        return "";
    }

    // TODO: get this writing to a file
    fn write(&mut self, records: &RecordStore) {
       for (string, fields) in records.items() {
           println!("{}", string);
       } 
    }
}

pub struct JsonWriter {
    output: String
}

impl RecordStoreWriter for JsonWriter {
    fn new() -> JsonWriter {
        JsonWriter { output: String::new() }
    }

    fn get_output(&self) -> &str {
        return &self.output;
    }

    fn write(&mut self, records: &RecordStore) {
        let mut result = String::new();
        result += "{";
        for (string, fields) in records.items() {
            result += &format!("\"{}\":", string);
            result += "{";
            result += &fields.iter()
                .map(|(field_name, field_val)|
                    format!("\"{}\": \"{}\"", field_name, field_val))
                .join(",");
            result += "}";
        }
        result += "}";
        self.output = result;
    }
}

pub struct ConsoleWriter {}

impl RecordStoreWriter for ConsoleWriter {
    fn new() -> ConsoleWriter {
        ConsoleWriter {}
    }

    // TODO: fill this in 
    fn get_output(&self) -> &str {
        return "";
    }

    fn write(&mut self, records: &RecordStore) {
        println!("{:?}", records);
    }
}