#![cfg_attr(not(debug_assertions),allow(non_snake_case,non_upper_case_globals,non_camel_case_types))]
#![cfg_attr(    debug_assertions ,allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros))]
extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;

_mod!(binmod); //→ #[path="binmod/[binmod].rs"] pub mod binmod;
use crate::binmod::print42;

use std::error::Error;
use std::result;

use tracing_subscriber::prelude::*; // added error check
use tracing_oslog::OsLogger;
const log_subsystem:&'static str = "xtrash";
const log_category :&'static str = "tool";
pub fn setup_os_log() -> Result<()> {
  let collector = tracing_subscriber::registry().with(OsLogger::new(log_subsystem,log_category));
  tracing::subscriber::set_global_default(collector).expect("failed to set global subscriber"); //⚠️ libs should avoid this to not cause conflicts when executables that depend on the library try to set the default later
  Ok(())
}
type Result<T> = result::Result<T, Box<dyn Error>>;
fn main() -> Result<()> {
  print42()?;
  Ok(())
}
