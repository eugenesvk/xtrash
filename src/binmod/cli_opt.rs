use crate::*;
use crate::binmod::{xattr_key,xattr_batch};

use bpaf	::{*, long as l, short as s, positional as pos}; // short names to allow starting builders
use super::bpaf_ext::*;

#[derive(Debug,Clone)] pub struct Opt {pub undo:bool, pub group:bool, pub skip_c:bool, pub api:Option<String>, pub paths:Vec<PathBuf>,}

use owo_colors::OwoColorize;
pub fn options() -> OptionParser<Opt> {
  let undo	= s('u').l("undo"   ).h({let mut d = Doc::default();d.text("⎌ Undo trashing, path to:\n ");
    d.emphasis("• ");d.text("‘");d.text(xattr_batch);d.text("_15꞉01꞉17_123’-styled batch subdir in 🗑 to restore all of its items\n ");
    d.emphasis("• ");d.text("a single file in 🗑 to restore it\n ");
    d.emphasis("• ");d.text("TBD parent dir to restore any children still in 🗑 that were removed from that dir\n ");
    d.emphasis("• ");d.text("TBD a single file to restore if any found in 🗑 (latest removed is restored if multiple)\n ");
    d.literal("r");d.text("|");d.literal("restore");d.text(" (alias)");
    d}).      	  s('r').l("restore").switch();
  // let group	= s('g').l("group"   ).h(&*format!("Move all items to a ‘{}_15꞉01꞉17_123’-styled subdir (alias: b̲atch)",xattr_batch))
  let group   	= s('g').l("group"   ).h({let mut d = Doc::default();d.text("Move all items to a ‘");d.text(xattr_batch);d.text("_15꞉01꞉17_123’-styled subdir \n ");
    d.literal("b");d.text("|");d.literal("batch");d.text(" (alias)");
    d}).	  s('b').l("batch").switch();
  use std::sync::Arc;
  let hh =format!("{}|{}|{}","d".blue().bold(),"f".blue().bold(),"os".blue().bold());
  let hh = Box::leak(Box::new(hh)); //TODO: leak to make static, couldn't make it work with ArcStr crate though it's supposed to work as &str

  let skip_c	= s('c').l("skipc" ).h("(TBD) Add the number of skipped items to the group subdir's name").env("xtrash_skip_c").switch();
  let api   	= s('v').l("via"   ).h({let mut d = Doc::default();d.text("(TBD) Trashing method to use (case insensitive):\n ");
    d.literal("d" );d.text(" | ");d.literal("Direct");d.text(": move items directly (Undo only via this tool)\n ");
    d.literal("f" );d.text(" | ");d.literal("Finder");d.text(": use Finder (OS undo works)\n ");
    d.literal("os");d.text(": use NsFileManager api (OS undo bugs)");
    d}).env("xtrash_via").argument::<String>("METHOD").optional();
    // direct ANSI styling breaks width calculations
    // d}).env("xtrash_via").argument::<String>(hh).optional();
    // d.text(&format!("{}","d".blue().bold()));d.text(" | ");d.text(&format!("{}","Direct".blue().bold()));d.text(": move items directly (Undo only via this tool)\n ");
    // d.text(&format!("{}","f".blue().bold()));d.text(" | ");d.text(&format!("{}","Finder".blue().bold()));d.text(": use Finder (OS undo works)\n ");
    // d.text(&format!("{}","os".blue().bold()));d.text(": use NsFileManager api (OS undo bugs)");
  let paths	= pos::<PathBuf>("PATH").some("Expecting paths to dir/file(s)… (run with -h for help)");
  construct!(Opt {undo,group,skip_c,api,paths}).to_options()
    .version(env!("CARGO_PKG_VERSION"))
    .descr("Move dir/file(s) to 🗑 ‘~/.Trash’ or restore previously trashed ones (with this tool)")
    // .header("")
    .footer(&*format!("(items are tracked via extended attributes keyed with ‘{}’, batch dirs — with ‘{}’)",xattr_key,xattr_batch))
    .with_usage(|doc| {let mut u = Doc::default();/*u.emphasis("Use");u.text(": ");*/
      u.literal(env!("CARGO_BIN_NAME"));u.text(" ");u.doc(&doc);
      u
    })
}
