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

pub fn trash_all<P:AsRef<Path>>(cc_paths:&[P], group:bool, skip_c:bool, api:DeleteMethod) -> result::Result<HashMap<String,Vec<&Path>>,ErTrash> {
  let safe_undo = false; //todo: add user arg: aborts and returns error if cant't set Xattr even if move was successful, otherwise we can't undo without xattr
  let safe_create = false; //todo: add user arg: aborts and returns error if cant't create a unique target at trash

  let mut skipped:HashMap<String,Vec<&Path>> = HashMap::new(); //todo push lists later, group by error type
  let mut skipped_name :Vec<&Path> = vec![]; // skipped due to unresolved file name
  let mut skipped_par  :Vec<&Path> = vec![]; // skipped due to unresolved parent or canonicalization error
  let mut skipped_trash:Vec<&Path> = vec![]; // skipped due to already being in trash
  let mut skipped_dupe :Vec<&Path> = vec![]; // skipped due to dupe and inability to create a unique file

  #[allow(deprecated)] let home_dir = match env::home_dir() {
    Some(path)	=> path,  //todo check if empty ""
    None      	=> return Err(ErTrash::NoHome),
  };
  let mut p = PathBuf::new();p.push(home_dir.clone());p.push(TRASH); let trash_path = p; // /Users/x/.Trash

  // 1. Create a "parent group" dir in Trash if ‘group’ (easier to differentiate from other trash), otherwise use Trash as the base
  let mut trash_parent = trash_path.clone(); // store successfully created group dir in trash
  if group { let imax = 50;
    for i in 1..=imax {
      let mut dir_g_name = group_timed(); if i>5 {dir_g_name.push('_');dir_g_name.push_str(&i.to_string())}
      let mut trash_par = trash_path.clone(); trash_par.push(dir_g_name); // /Users/x/.Trash/xtrash_g_15꞉01꞉17_123
      match fs::create_dir(&trash_par) {//debug!("✓ trash_par = {:?}",trash_par)?;
        Ok (()) => {
          match xattr::set(&trash_par,xattr_batch,&[1]) {
            Ok (()) => {debug!("Created group subdir and set its extended attributes: {:?}",trash_par)},
            Err(e ) => {if safe_undo {return Err(ErTrash::NoXattr(trash_par))} else {error!("No Undo: created group subdir ‘{:?}’, but failed to set its extended attributes due to: {:?}",trash_par,e)}},
          }
          trash_parent = trash_par;
          break},
        Err(e ) => {if i==imax {return Err(ErTrash::IoTrashPar{i:i,path:trash_par,e:e})}},
      }
    }
  }
  let total_count = cc_paths.len();

  let imax = 100;
  for path in cc_paths {
    // todo: bail if path doesn;t exit, add to skipped
    let path     	= path.as_ref(); // /Users/x/Documents/1.txt
    let base_name	= match path.file_name() { //1.txt
      Some(p)    	=> p,
      None       	=> {skipped_name.push(path); continue}};
    let stem     	= path.file_stem().unwrap_or_else(|| OsStr::new("")); //1
    let ext      	= match path.extension() { // .txt (.ext in rust returns without a dot)
      Some(ex)   	=> &concat_2oss(".",ex),
      None       	=> OsStr::new(""),};
    let parent   	= match path.parent   () { // /Users/x/Documents
      Some(p)    	=> {
        let canon	= match p.canonicalize() {
          Ok (pc)	=> pc,
          Err(e )	=> {skipped_par.push(path); continue}};
        canon    	},
      None       	=> {skipped_par.push(path); continue}};
    let is_real_dir =  path.is_dir    ()
      &&             ! path.is_symlink();


    // let mut p = PathBuf::new();p.push(home_dir   .clone());p.push(TRASH    ); let trashed_par   = p;

    // 2. Find a suitable target path in Trash (append time if dupe)
    let mut trashed_path = trash_parent.clone(); trashed_path.push(base_name); // /Users/x/.Trash/xtrash_g_15꞉01꞉17_123/src_path (or without xtrash...)
    let mut trashed_file:Option<File> = None;
    for i in 1..=imax {
      if is_real_dir { // check if available by creating a dir (will be replaced later with our source)
        match fs::create_dir(&trashed_path) {//debug!("✓ dir trashed_path = {:?}",trashed_path)?;
          Ok (()) =>
            break,
          Err(e ) => {if i==imax {if safe_create {return Err(ErTrash::IoTrashDest{i:i,src:path.to_path_buf(),dst:trashed_path,e:e})
              } else {error!("I/O error when preparing to move ‘{:?}’ to ‘{:?}’ in 🗑 (tried {i} variants), failed to create a unique dir: ‘{e}’",path,trashed_path); skipped_dupe.push(path);}
            } else {
              let mut time_pad = time_s(' '); if i>5 {time_pad.push('_');time_pad.push_str(&i.to_string())}
              trashed_path.pop(); trashed_path.push(concat_2oss(base_name,time_pad));}
            continue},
        }
      } else { // ↓ atomic, avoids TOCTOU race condition
        match fs::File::create_new(&trashed_path) { //debug!("✓ file trashed_path = {:?}",trashed_path)?;
          Ok (f ) => {trashed_file = Some(f);
            break},
          Err(e ) => {if i==imax {if safe_create {return Err(ErTrash::IoTrashDest{i:i,src:path.to_path_buf(),dst:trashed_path,e:e})
              } else {error!("I/O error when preparing to move ‘{:?}’ to ‘{:?}’ in 🗑 (tried {i} variants): failed to create a unique file: ‘{e}’",path,trashed_path); skipped_dupe.push(path);}
            } else {
              let mut time_pad = time_s(' '); if i>5 {time_pad.push('_');time_pad.push_str(&i.to_string())}
              let file_name_padded = concat_oss(&[stem,time_pad.as_ref(),ext]).unwrap_or_else(|e| concat_2oss(base_name,time_pad));
              trashed_path.pop(); trashed_path.push(file_name_padded);}
            continue},
        }
      }
    }

    // 3. Move to the found target
    let dir_file = if is_real_dir{"dir"}else{"file"}; //🗀📁🗁 🗋🗎
    match fs::rename(&path,&trashed_path) {
      Ok (())     => {debug!("✓ to move {} to trash!: {:?} to {:?}",dir_file, &path, &trashed_path);
        match xattr::set(&trashed_path,xattr_key,path.as_os_str().as_encoded_bytes()) {
          Ok (()) => {debug!("✓ set xattr to moved {} at trash!: {:?} to {:?}",dir_file, &path, &trashed_path)},
          Err(e ) => {if safe_undo {return Err(ErTrash::NoXattr(trashed_path))
            } else {  error!("No Undo: created {} ‘{:?}’, but failed to set its extended attributes due to: {:?}",dir_file,trashed_path,e)}},
        }
      },
      Err(e ) => {match e.kind() {
        ErrorKind::NotFound         => {warn!("Failed to move {} to trash: {:?}",dir_file,e);}
        ErrorKind::PermissionDenied => {warn!("Failed to move {} to trash: {:?}",dir_file,e);} //TODO: request sudo
        // ErrorKind::CrossesDevices => {warn!("failed to move {} to trash!: {:?}",dir_file,e);} //unstable TODO: use another method
        _ => {error!("Failed to move {} to trash: {:?}",dir_file,e);}
        }
        if ! is_real_dir { //
          if let Some(f) = trashed_file {
            if let Err(e) = fs::remove_file(&trashed_path) {error!("Failed to cleanup after ourselves — removing an empty file {:?}: {:?}",&trashed_path,e)}
          }
        }
      },
    }
  } // end of loop over paths

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
