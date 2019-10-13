extern crate itertools;

use crate::pm::recordstore::RecordStore;
use crate::pm::macros;
use itertools::Itertools;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

// TODO: this can probably be generalized further via closures to avoid an if let
fn apply_decorator<'a, T: RecordStoreWriter>(records: &RecordStore,
    decorator: &'a mut T) -> Option<&'a str> {
        let mut nw = NullWriter::new();
        decorator.write(records, &mut nw);
        decorator.get_output()
}

pub trait RecordStoreWriter {
    fn new() -> Self;

    fn get_output(&self) -> Option<&str>;

    fn write<T : RecordStoreWriter>(&mut self,
        records: &RecordStore, decorator: &mut T);
}

pub struct NullWriter {}

impl RecordStoreWriter for NullWriter {
    fn new() -> NullWriter {
        NullWriter {}
    }

    fn get_output(&self) -> Option<&str> {
        None
    }

    fn write<T : RecordStoreWriter>(&mut self,
        records: &RecordStore, decorator: &mut T) {
    }
}

pub struct FileWriter {
    file_name: String,
    output: String
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
        FileWriter { file_name: String::new(),
            output: String::new() }
    }

    // TODO: fill this in 
    fn get_output(&self) -> Option<&str> {
        Some(&self.output)
    }

    // TODO: get this writing to a file
    fn write<T : RecordStoreWriter>(&mut self,
        records: &RecordStore, decorator: &mut T) {
        if let Some(dec_output) = apply_decorator(records, decorator) {
            self.output = String::from(dec_output);
        } else {
            self.output = format!("{:?}", records);
        }
        macros::write_all_to_file(&self.file_name, &self.output);
    }
}

pub struct JsonWriter {
    output: String
}

impl RecordStoreWriter for JsonWriter {
    fn new() -> JsonWriter {
        JsonWriter { output: String::new() }
    }

    fn get_output(&self) -> Option<&str> {
        Some(&self.output)
    }

    fn write<T: RecordStoreWriter>(&mut self,
        records: &RecordStore, decorator: &mut T) {
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
    fn get_output(&self) -> Option<&str> {
        Some("")
    }

    fn write<T: RecordStoreWriter>(&mut self,
        records: &RecordStore, decorator: &mut T) {
        println!("{:?}", records);
    }
}