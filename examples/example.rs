use std::collections::HashMap;

use schema_macro_thing::*;

trait Schema {
    fn get_schema() -> HashMap<String, String>;
}

#[allow(unused)]
#[derive(Schema)]
struct MyStruct {
    field1: usize,
    field2: bool,
    field3: Option<i32>,
}

fn main() {
    println!("{:#?}", MyStruct::get_schema())
}
