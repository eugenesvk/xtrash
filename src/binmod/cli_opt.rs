use crate::*;
use crate::binmod::{xattr_key,xattr_group};

use bpaf	::{*, long as l, short as s, positional as pos}; // short names to allow starting builders
use super::bpaf_ext::*;

#[derive(Debug,Clone)] pub struct Opt {undo:bool, paths:Vec<PathBuf>,}

pub fn options() -> OptionParser<Opt> {
  let undo	= s('u').l("undo"   ).h("⎌ Undo trashing (alias: r̲estore): path to
    1. ‘xtrash_15꞉01꞉17_123’-styled batch dir in 🗑 to restore all of its items
    2. a single file in 🗑 to restore it
    3. TBD parent dir to restore any children still in 🗑 that were removed from that dir
    4. TBD a single file to restore if any found in 🗑 (latest removed is restored if multiple)")
    .      	  s('r').l("restore").switch();
  let paths	= pos::<PathBuf>("PATH").some("Expecting paths to dir/file(s)…");
  construct!(Opt {undo,paths}).to_options()
    .version(env!("CARGO_PKG_VERSION"))
    .descr("Move dir/file(s) to 🗑 ‘~/.Trash’ or restore previously trashed ones (with this tool)")
    // .header("")
    .footer(&*format!("(items are tracked via extended attributes keyed with ‘{}’, batch dirs — with ‘{}’)",xattr_key,xattr_group))
}
