pub fn ret42() -> i32 { 42 }
use std::{env, result, error::Error,};

type Result<T> = result::Result<T, Box<dyn Error>>;

use users::{get_current_uid,uid_t};
use lazy_static::lazy_static;
lazy_static! {pub static ref UID : uid_t  = get_current_uid();}
lazy_static! {pub static ref UIDS: String = UID.to_string  ();}

use sudo::RunningAs;
/// Get User ID of the logged in user (=user owning console in this case) when running as root, otherwise returns proc UID
pub fn get_logged_uid() -> Result<uid_t> {
  if let RunningAs::User = sudo::check() {return Ok(*UID)};
  if let   Ok(sudo_uid) = env::var("SUDO_UID") {//trace!("SUDO_UID found {}",&sudo_uid);
    return Ok(sudo_uid.parse::<uid_t>()?)
  } else {Err("SUDO_UID missing even though the process is not running as User".into())}
}

/// TODO: when objc supports SystemConfiguration, port this Swift code to replace the env var check above
pub fn _get_logged_uid() -> Option<uid_t> {
  // alternative loggedInUser=$(stat -f %Su /dev/console), check edge cases scriptingosx.com/2020/02/getting-the-current-user-in-macos-update
  let swift_code = r#"
  import SystemConfiguration
  func getConsoleUser() -> String? {
    let store = SCDynamicStoreCreate(nil, "xtrash.consoleUserID" as CFString, nil, nil)
    var uid: uid_t = 0 // → UnsafeMutablePointer<uid_t>
    var gid: gid_t = 0 // → UnsafeMutablePointer<gid_t>
    let ret = SCDynamicStoreCopyConsoleUser(store,&uid,&gid) as String?
    print("uid",uid as Any) //
    print("gid",gid as Any) //
    return ret
  }
  "#;
  None
}

/// Set process ID to that of the logged in user (to not trash to root trash, but user's trash, but then you might have no permissions)
pub fn _unsudo() {
  if let   Ok(regu_uid) = get_logged_uid() {//trace!("setting regular {}",&regu_uid);
    unsafe { libc::setuid(regu_uid); }
  }
}
