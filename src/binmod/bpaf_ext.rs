use std::str    	::FromStr;
use bpaf        	::{*, long as l, short as s, positional as pos}; // short names to allow starting builders
use bpaf::params	::{NamedArg, ParseArgument, ParsePositional};
pub trait BpafAlias { // add wrapper trait to allow using shorter .l options to continue builders
  fn s     (self, short:char        ) -> Self                                      	;
  fn l     (self, long :&'static str) -> Self                                      	;
  fn h  <M>(self, help :M           ) -> Self             where M:Into<Doc>        	;
  fn arg<T>(self, arg  :&'static str) -> ParseArgument<T> where T:FromStr + 'static	; }
impl      BpafAlias for NamedArg {
  fn s     (self, short:char        ) -> Self                                       {self.short   (short)}
  fn l     (self, long :&'static str) -> Self                                       {self.long    (long )}
  fn h  <M>(self, help :M           ) -> Self             where M:Into<Doc>         {self.help    (help )}
  fn arg<T>(self, arg  :&'static str) -> ParseArgument<T> where T:FromStr + 'static {self.argument(arg  )} }
pub trait BpafAliasPos { // ... for positional arguments
  fn h  <M>(self, help :M           ) -> Self             where M:Into<Doc>	;  }
impl<T>   BpafAliasPos for ParsePositional<T> {
  fn h  <M>(self, help :M           ) -> Self             where M:Into<Doc>         {self.help    (help )}
}

use bpaf::doc::Style;
use bpaf::Doc;
pub trait BpafDocAlias { // add wrapper trait to allow using shorter .em (instead of .emphasis) Doc options to continue builders
  fn b  (&mut self, text:&str) {}
  fn em (&mut self, text:&str) {}
  fn lit(&mut self, text:&str) {}
  fn x  (&mut self, text:&str) {}
  fn inv(&mut self, text:&str) {}
}
impl      BpafDocAlias for Doc {
  fn b  (&mut self, text:&str) {self.emphasis(text);}
  fn em (&mut self, text:&str) {self.emphasis(text);}
  fn lit(&mut self, text:&str) {self.literal (text);}
  fn x  (&mut self, text:&str) {self.invalid (text);}
  fn inv(&mut self, text:&str) {self.invalid (text);}
}
