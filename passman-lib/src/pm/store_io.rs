#![feature(inner_deref)]

extern crate itertools;

use crate::pm::recordstore::RecordStore;
use crate::pm::macros;
use itertools::Itertools;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;


// TODO: this can probably be generalized further via closures to avoid an if let
fn apply_decorator<'a, T: RecordStoreWriter, F>(records: &RecordStore,
    decorator: &'a mut T, behavior: F) -> Option<String> 
    where F: Fn(&RecordStore) -> Option<String> {
        let mut nw = NullWriter::new();
        decorator.write(records, &mut nw);
        if let Some(str_output) = decorator.get_output() {
            Some(String::from(str_output))
        } else {
            behavior(records)
        }
}

// usage: create writer that will be at the end of the chain
// writers are executed earlier the more nested they are
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
    output: Option<String>
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
            output: None }
    }

    fn get_output(&self) -> Option<&str> {
        self.output.as_deref()
    }

    // TODO: get this writing to a file
    fn write<T : RecordStoreWriter>(&mut self,
        records: &RecordStore, decorator: &mut T) {
        self.output = apply_decorator(records, decorator,
            |r| Some(format!("{:?}", r)));
        if let Some(output_str) = &self.output {
            // TODO: check IO result
            macros::write_all_to_file(&self.file_name, &output_str);
        }
    }
}

pub struct JsonWriter {
    output: Option<String>
}

impl RecordStoreWriter for JsonWriter {
    fn new() -> JsonWriter {
        JsonWriter { output: None }
    }

    fn get_output(&self) -> Option<&str> {
        self.output.as_deref()
    }

    fn write<T: RecordStoreWriter>(&mut self,
        records: &RecordStore, decorator: &mut T) {
        fn to_json(r: &RecordStore) -> Option<String> {
            let mut result = String::new();
            result += "{";
            for (string, fields) in r.items() {
                result += &format!("\"{}\":", string);
                result += "{";
                result += &fields.iter()
                    .map(|(field_name, field_val)|
                        format!("\"{}\": \"{}\"", field_name, field_val))
                    .join(",");
                result += "}";
            }
            result += "}";
            Some(result)
        }
        // TODO: determine if there are cases where we want to
        // format as JSON in addition to decorator, rather than
        // either or
        self.output = apply_decorator(records,
            decorator, to_json);
    }
}

pub struct ConsoleWriter {
    output: Option<String>
}

impl RecordStoreWriter for ConsoleWriter {
    fn new() -> ConsoleWriter {
        ConsoleWriter {
            output: None
        }
    }

    fn get_output(&self) -> Option<&str> {
        self.output.as_deref()
    }

    fn write<T: RecordStoreWriter>(&mut self,
        records: &RecordStore, decorator: &mut T) {
        self.output = apply_decorator(records, decorator,
            |r| Some(format!("{:?}", r)));
        // should be guaranteed to unwrap
        println!("{}", self.get_output().unwrap());
    }
}