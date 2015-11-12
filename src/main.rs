#![feature(convert)]

extern crate yaml_rust;
extern crate regex;

#[macro_use]
extern crate lazy_static;

mod helpers;

use yaml_rust::{YamlLoader, YamlEmitter};

fn main() {
    // let s = 
    // "
    // foo: 
    //   - list1
    //   - list2
    // bar:
    //   - 1
    //   - 2.0
    // ";

    // let docs = YamlLoader::load_from_str(s).unwrap();

    // let doc = &docs[0];
    // println!("{:?}", doc);

    // assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    // assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

    // assert!(doc["INVALID_KEY"][100].is_badvalue());

    // let mut out_str = String::new();
    // {
    //     let mut emitter = YamlEmitter::new(&mut out_str);
    //     emitter.dump(doc).unwrap();
    // }
    // println!("{}", out_str);
}