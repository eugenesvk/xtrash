use crate::*;
use crate::binmod::{xattr_key,xattr_batch};

use bpaf	::{*, long as l, short as s, positional as pos}; // short names to allow starting builders
use super::bpaf_ext::*;

#[derive(Debug,Clone)] pub struct Opt {pub group:bool, pub undo:bool, pub paths:Vec<PathBuf>,}

pub fn options() -> OptionParser<Opt> {
  let group	= s('g').l("group"   ).h(&*format!("Move all items to a ‘{}_15꞉01꞉17_123’-styled subdir (alias: b̲atch)",xattr_batch))
    .      	  s('b').l("batch").switch();
  let undo 	= s('u').l("undo"   ).h(&*format!("⎌ Undo trashing (alias: r̲estore): path to
    1. ‘{}_15꞉01꞉17_123’-styled batch subdir in 🗑 to restore all of its items
    2. a single file in 🗑 to restore it
    3. TBD parent dir to restore any children still in 🗑 that were removed from that dir
    4. TBD a single file to restore if any found in 🗑 (latest removed is restored if multiple)",xattr_batch))
    .      	  s('r').l("restore").switch();
  let paths	= pos::<PathBuf>("PATH").some("Expecting paths to dir/file(s)… (run with -h for help)");
  construct!(Opt {group,undo,paths}).to_options()
    .version(env!("CARGO_PKG_VERSION"))
    .descr("Move dir/file(s) to 🗑 ‘~/.Trash’ or restore previously trashed ones (with this tool)")
    // .header("")
    .footer(&*format!("(items are tracked via extended attributes keyed with ‘{}’, batch dirs — with ‘{}’)",xattr_key,xattr_batch))
    .with_usage(|doc| {let mut u = Doc::default();/*u.emphasis("Use");u.text(": ");*/
      u.literal(env!("CARGO_BIN_NAME"));u.text(" ");u.doc(&doc);
      u
    })
}
