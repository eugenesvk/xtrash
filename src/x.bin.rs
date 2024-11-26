#![cfg_attr(not(debug_assertions),allow(non_snake_case,non_upper_case_globals,non_camel_case_types))]
#![cfg_attr(    debug_assertions ,allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros))]
extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;

_mod!(binmod); //→ #[path="binmod/[binmod].rs"] pub mod binmod;
use crate::binmod::{print42,main_cli};

use std::error::Error;
use std::result;

use std::path::{Path,PathBuf};

use tracing_subscriber::prelude::*; // added error check
use tracing_oslog::OsLogger;
const log_subsystem:&'static str = "xtrash";
const log_category :&'static str = "tool";
pub fn setup_os_log() -> Result<()> {
  let collector = tracing_subscriber::registry().with(OsLogger::new(log_subsystem,log_category));
  tracing::subscriber::set_global_default(collector).expect("failed to set global subscriber"); //⚠️ libs should avoid this to not cause conflicts when executables that depend on the library try to set the default later
  Ok(())
}

use once_cell::sync::Lazy;
pub static IS_TERM: Lazy<bool> = Lazy::new(|| io::stdout().is_terminal());
// pub static IS_CONSOLE: Lazy<bool> =
    // Lazy::new(|| io::stdin().is_terminal());
#[cfg(feature="cli")] use log    ::{*,trace as l4, debug as l3, info as l2, warn as l1, error as l0};
#[cfg(feature="gui")] use tracing::{*,trace as l4, debug as l3, info as l2, warn as l1, error as l0};

use std::io::IsTerminal;
type Result<T> = result::Result<T, Box<dyn Error>>;
fn main() -> Result<()> {try_main()}

fn try_main() -> Result<()> {
  #[cfg(feature="gui")] setup_os_log()?;
  #[cfg(feature="cli")] stderrlog::new().modules([module_path!()]).verbosity(4).init().unwrap();

  match main_cli() {
    Ok (())	=> {},
    Err(e )	=> error!("{}",e),
  }

  Ok(())
}
