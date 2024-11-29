extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;
use crate::*;

use std::error::Error;
use std::result;

pub mod bpaf_ext;
use bpaf_ext::*;
pub mod cli_opt;
use cli_opt::*;

type Result<T> = result::Result<T, Box<dyn Error>>;
pub fn print42() -> Result<()> {p!("{}",42)?; Ok(())}

use std::env;
use std::path::{Path,PathBuf};
use std::ffi::OsStr;

use xattr;
// use anyhow::{anyhow, Result, bail};

static xattr_key  	:&str = "xtrash_orig_path";
static xattr_batch	:&str = "xtrash";
static TRASH      	:&str = ".Trash";

static _dbg:i8 = 1;
/// Quick and dirty way to disable blocks of debug-level code, use `if _d(1) {}` to do something only if global _dbg ≥ 1
pub fn _d(lvl:i8) -> bool {if _dbg>=lvl{true}else{false}}

use std::collections::HashMap;

use std::io::Read;
use thiserror::Error;
#[derive(Debug,Error)] pub enum ErTrash {
  #[error("I/O error creating a group subdir ‘{path}’ (tried {i} variants) in 🗑 : {e}")]
  IoTrashPar {i:u8,path:PathBuf, e:io::Error,},
  #[error("I/O error when preparing to move ‘{src}’ to ‘{dst}’ in 🗑 (tried {i} variants) : {e}")]
  IoTrashDest {i:u8,src:PathBuf,dst:PathBuf, e:io::Error,},
  #[error("No Undo: Failed to set extended attributes for ‘{0}’")]
  NoXattr(PathBuf),
  #[error("Found no $HOME environment variable")]
  NoHome,
  #[error("Unrecognized cli arguments")]
  BadArg,
}

use std::io::IsTerminal;

pub fn main_cli() -> Result<()> {
  let opt = options().run();
  // println!("{:#?}", opt);

  let pth:PathBuf = PathBuf::new();
  let pth2:&Path = Path::new("./foo/bar.txt");
  let cc_paths:Vec<&Path> = vec![&pth,pth2];
  match trash_all(&cc_paths, opt.group) {
    Ok (()) => {},
    Err(e) => pe!("{e}")?,
  }

  Ok(())
}

use time::OffsetDateTime;
pub fn time_s() -> String {
  let now = match OffsetDateTime::now_local() {Ok (t) => t,
    Err(_) =>     OffsetDateTime::now_utc  (),};
  format!("{:#02}꞉{:#02}꞉{:#02}_{:#03}",now.hour(),now.minute(),now.second(),now.millisecond())
}
pub fn group_timed() -> String {format!("{}_{}",xattr_batch,time_s())}
use std::fs;

pub fn trash_all<P:AsRef<Path>>(cc_paths:&[P], group:bool) -> result::Result<(),ErTrash> {
  // let mut skipped:HashMap<String,Vec<PathBuf>> = HashMap::new(); //todo push lists later, group by error type
  let mut skipped_nm   :Vec<PathBuf> = vec![]; // skipped due to unresolved file name
  let mut skipped_par  :Vec<PathBuf> = vec![]; // skipped due to unresolved parent or canonicalization error
  let mut skipped_trash:Vec<PathBuf> = vec![]; // skipped due to already being in trash

  #[allow(deprecated)] let home_dir = match env::home_dir() {
    Some(path)	=> path,  //todo check if empty ""
    None      	=> return Err(ErTrash::NoHome),
  };
  let mut p = PathBuf::new();p.push(home_dir.clone());p.push(TRASH); let trash_path = p; // /Users/x/.Trash

  // 1. Create a "parent group" dir in Trash if ‘group’ (easier to differentiate from other trash), otherwise use Trash as the base
  let mut trash_parent = trash_path.clone(); // store successfully created group dir in trash
  if group { let imax = 50;
    for i in 1..=imax {
      let mut dir_g_name = group_timed(); if i>5 {dir_g_name.push_str(&i.to_string())}
      let mut trash_par = trash_path.clone(); trash_par.push(dir_g_name); // /Users/x/.Trash/xtrash_g_15꞉01꞉17_123
      match fs::create_dir(&trash_par) {//debug!("✓ trash_par = {:?}",trash_par)?;
        Ok (()) => {if ! xattr::set(&trash_par,xattr_batch,&[1]).is_ok() {return Err(ErTrash::NoXattr(trash_par))}
          trash_parent = trash_par;
          break},
        Err(e ) => {if i==imax {return Err(ErTrash::IoTrashPar{i:i,path:trash_par,e:e})}},
      }
    }
  }

  let mut path_counts:HashMap<PathBuf,u8> = HashMap::new();
  let batch_fd = 1;
  for path in cc_paths {
    let path = path.as_ref(); // /Users/x/Documents/1.txt
    let file_name = match path.file_name() { //1.txt
      Some(p)    	=> p,
      None       	=> {skipped_nm.push(path.into()); continue}};
    let parent   	= match path.parent      () { // /Users/x/Documents
      Some(p)    	=> {
        let can  	= match p   .canonicalize() {
          Ok (pc)	=> pc,
          Err(e )	=> {skipped_par.push(path.into()); continue}};
        can },
      None	=> {skipped_par.push(path.into()); continue}};

    let mut p = PathBuf::new();p.push(parent);             p.push(file_name); let resolved_path = p; // Resolve sym🔗 in dirs, but not the file itself
    if resolved_path.starts_with(&trash_path) {skipped_trash.push(path.into()); continue} // already in trash


    // let mut p = PathBuf::new();p.push(home_dir   .clone());p.push(TRASH    ); let trashed_par   = p;

    let mut p = PathBuf::new();p.push(home_dir   .clone());p.push(TRASH    ); let trashed_par   = p;
    let mut p = PathBuf::new();p.push(trashed_par);p.push(file_name); let trashed_path  = p;
    if _d(1){debug!("trashed_path = {:?} resolved_path={:?}",trashed_path,resolved_path);}


  // Cleanup
  if group { match fs::remove_dir(&trash_parent) {
    Ok (()) => {debug!("Cleaned up empty group subdir {:?}", &trash_parent)}, // if skipped all files
    Err(e ) => { match e.kind() {
      ErrorKind::NotFound => {error!("")}, //
      ErrorKind::PermissionDenied => {error!("")}, // TODO: uncomment when stabilized
      // ErrorKind::DirectoryNotEmpty => {debug!("")}, // ok to ignore: The directory isn't empty
      // ErrorKind::NotADirectory => {error!("")}, // not ok to ignore
      // _ => error!("Failed to cleanup after ourselves — removing an empty group dir {:?}: {:?}",&trash_parent,e), //
      _ => {}, //combines 2 unstable errorkinds
      }
      if skip_c { // append skipe count if we haven't removed the empty dir
        let l_name 	= skipped_name .len();
        let l_par  	= skipped_par  .len();
        let l_trash	= skipped_trash.len();
        let l_dupe 	= skipped_dupe .len();
        let skip_count = l_name + l_par + l_trash + l_dupe;
        if skip_count > 0 {let mut lbl = String::new();
          if l_name 	> 0 {lbl.push_str(&format!(" name{}" ,l_name ))};
          if l_par  	> 0 {lbl.push_str(&format!(" par{}"  ,l_par  ))};
          if l_trash	> 0 {lbl.push_str(&format!(" trash{}",l_trash))};
          if l_dupe 	> 0 {lbl.push_str(&format!(" dupe{}" ,l_dupe ))};
          debug!("skipped {} from {} in cat: {}",skip_count,total_count,lbl);
          let trash_parent_count = concat_2oss(&trash_parent,lbl);
          if let Err(e) = fs::rename(&trash_parent,&trash_parent_count) {error!("Failed to append skipped count to our group subdir: {:?}",e)};
        }
      }
    }}
  }

  // Return all skipped paths by category for future logging
  skipped.insert("name"  	.to_string(),skipped_name );
  skipped.insert("parent"	.to_string(),skipped_par  );
  skipped.insert("trash" 	.to_string(),skipped_trash);
  skipped.insert("dupe"  	.to_string(),skipped_dupe );
  Ok(skipped)
}
