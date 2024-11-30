#![cfg_attr(not(debug_assertions),allow(non_snake_case,non_upper_case_globals,non_camel_case_types))]
#![cfg_attr(    debug_assertions ,allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros))]

#[cfg(feature="cli")] use log    ::{*,trace as l4, debug as l3, info as l2, warn as l1, error as l0};
#[cfg(feature="gui")] use tracing::{*,trace as l4, debug as l3, info as l2, warn as l1, error as l0};

#[test]
fn from_utf8_lossy_pc() {
  use crate::from_utf8_lossy_pc;
  assert_eq!(          "foo%F1%80bar"
  ,from_utf8_lossy_pc(b"foo\xF1\x80bar"),);
  assert_eq!(          "foo%F8bar" //ø
  ,from_utf8_lossy_pc(b"foo\xF8bar"),);
}
