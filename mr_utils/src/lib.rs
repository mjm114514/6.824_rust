#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct KeyValue {
  pub key: String,
  pub value: String,
}