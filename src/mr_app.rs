use libloading::{ Library, Symbol };
use mr_utils::KeyValue;
use std::path::Path;

#[cfg(libloading_docs)]
use libloading::os::unix as imp; // the implementation used here doesn't matter particularly much...
#[cfg(all(not(libloading_docs), unix))]
use libloading::os::unix as imp;
#[cfg(all(not(libloading_docs), windows))]
use libloading::os::windows as imp;

pub struct MapReduceApp {
  _lib: Library,
  map_raw: imp::Symbol<fn(String, String) -> Vec<KeyValue>>,
  reduce_raw: imp::Symbol<fn(String, Vec<String>) -> String>,
}

impl MapReduceApp {
  pub fn new(path: &Path) -> Self {
    let lib = unsafe { Library::new(path).unwrap() };
    let map_raw = unsafe {
      let map: Symbol<fn(String, String) -> Vec<KeyValue>> =
        lib.get(b"map").unwrap();
      map.into_raw()
    };

    let reduce_raw = unsafe {
      let reduce: Symbol<fn(String, Vec<String>) -> String> =
        lib.get(b"reduce").unwrap();
      reduce.into_raw()
    };

    MapReduceApp {
      _lib: lib,
      map_raw,
      reduce_raw,
    }
  }

  pub fn map(&self, filename: String, contents: String) -> Vec<KeyValue> {
    (self.map_raw)(filename, contents)
  }

  pub fn reduce(&self, key: String, values: Vec<String>) -> String {
    (self.reduce_raw)(key, values)
  }

}
