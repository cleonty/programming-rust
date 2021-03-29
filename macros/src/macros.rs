// macros.rs
#[macro_export]
macro_rules! json {
  (null) => {
    $crate::Json::Null
  };
  ([ $( $element:tt ),* ]) => {
    $crate::Json::Array(vec![ $( json!($element) ),* ])
  };
  ({ $( $key:tt : $value:tt ),* }) => {
    {
      let mut fields = ::std::boxed::Box::new(
      ::std::collections::HashMap::new());
      $( fields.insert(::std::string::ToString::to_string($key), json!($value)); )*
      $crate::Json::Object(fields)
    }
  };
  ($other:tt) => {
    $crate::Json::from($other)
  };
}