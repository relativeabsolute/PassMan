extern crate passman_lib;

use passman_lib::pm::recordstore;

fn main() {
    let mut record_store = recordstore::RecordStore::new();

    record_store.add_empty(String::from("record_name"));

    record_store.add_field("record_name",
        recordstore::Field(String::from("blah"), String::from("blahblah")));

    println!("{:?}", record_store);
}
