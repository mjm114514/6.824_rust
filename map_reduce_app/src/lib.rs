use mr_utils::KeyValue;

#[no_mangle]
pub fn map(filename: String, contents: String) -> Vec<KeyValue> {
  let words = contents.split(|x| !char::is_alphabetic(x));

  let kv_arr: Vec<KeyValue> = words
    .filter(|w| (*w).len() > 0)
    .map(|word| KeyValue {
      key: word.to_uppercase().into(),
      value: String::from("1"),
    })
    .collect();

  kv_arr
}

#[no_mangle]
pub fn reduce(key: String, values: Vec<String>) -> String {
  values.len().to_string()
}
