extern crate passman_lib;

use passman_lib::pm::recordstore;
use passman_lib::pm::store_io::{
    RecordStoreWriter, JsonWriter, NullWriter, ConsoleWriter};

fn main() {
    let mut record_store = recordstore::RecordStore::new();

    record_store.add_empty(String::from("record_name"));

    record_store.add_field("record_name",
        recordstore::Field(String::from("blah"), String::from("blahblah")));

    let mut writer = ConsoleWriter::new();

    writer.write(&record_store, &mut JsonWriter::new());

    // just to check if they are the same
    if let Some(output_str) = writer.get_output() {
        println!("{}", output_str);
    }
}
