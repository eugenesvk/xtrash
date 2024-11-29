pub fn ret42() -> i32 { 42 }
/// Set process ID to that of the logged in user (to not trash to root trash, but user's trash, but then you might have no permissions)
pub fn _unsudo() {
  if let   Ok(regu_uid) = get_logged_uid() {//trace!("setting regular {}",&regu_uid);
    unsafe { libc::setuid(regu_uid); }
  }
}
