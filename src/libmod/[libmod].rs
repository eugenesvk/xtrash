pub fn ret42() -> i32 { 42 }
use sudo::RunningAs;
/// Get User ID of the logged in user (=user owning console in this case) when running as root, otherwise returns proc UID
pub fn get_logged_uid() -> Result<uid_t> {
  if let RunningAs::User = sudo::check() {return Ok(*UID)};
  if let   Ok(sudo_uid) = env::var("SUDO_UID") {//trace!("SUDO_UID found {}",&sudo_uid);
    return Ok(sudo_uid.parse::<uid_t>()?)
  } else {Err("SUDO_UID missing even though the process is not running as User".into())}
}
/// Set process ID to that of the logged in user (to not trash to root trash, but user's trash, but then you might have no permissions)
pub fn _unsudo() {
  if let   Ok(regu_uid) = get_logged_uid() {//trace!("setting regular {}",&regu_uid);
    unsafe { libc::setuid(regu_uid); }
  }
}
