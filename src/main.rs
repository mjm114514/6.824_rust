mod mr_app;

use std::fs::File;
use std::{io::Write, path::Path};

use libloading::{Library, Symbol};
use mr_app::MapReduceApp;
use mr_utils::KeyValue;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 3 {
    eprintln!("Usage: xxx.so input files.");
    std::process::exit(1);
  }

  let mr_app = MapReduceApp::new(Path::new(&args[1]));

  let mut intermediate: Vec<KeyValue> = args[2..]
    .iter()
    .flat_map(|filename| {
      let contents = std::fs::read_to_string(filename).unwrap();
      mr_app.map(filename.clone(), contents)
    })
    .collect();

  intermediate.sort_by(|kvl, kvr| kvl.key.cmp(&(kvr.key)));

  let mut output_file = match File::create("mr-out-0") {
    Err(why) => panic!("couldn't create {}: {}", "mr-out-0", why),
    Ok(file) => file,
  };

  let mut i = 0;
  while i < intermediate.len() {
    let mut j = i + 1;
    while j < intermediate.len() && intermediate[j].key == intermediate[i].key {
      j += 1;
    }

    let values: Vec<String> = intermediate[i..j]
      .iter()
      .map(|kv| kv.value.clone())
      .collect();

    let output = mr_app.reduce(intermediate[i].key.clone(), values);

    match output_file
      .write_fmt(format_args!("{} {}\n", intermediate[i].key, output))
    {
      Err(why) => panic!("write to mr-out-0 failed: {}", why),
      Ok(size) => size,
    };

    i = j;
  }
}
