<p align="center">
(experimental)macOS command line utility to delete items to 🗑 with UNDO!
<br>
(since )
</p>

<p align="center">  
description continued
</p>


## Introduction

This utility deletes items to user's 🗑 trash and saves the original path in the extended attributes of the trashed items, allowing for later 

## Install

## Use

Avoid some of the dupe naming

  - `dupe_name.txt`
  - `dupe_name 14.20.15.txt`

issues by creating a group/batch directory to trash items to:
  - `~/.Trash/xtrash_14꞉20꞉01_945/dupe_name.txt`
  - `~/.Trash/xtrash_14꞉20꞉15_779/dupe_name.txt`

(dupe names within the same batch will still have a time-based suffix). This also helps with visually separating large batch deletes. Set via `-g` or `--group` argument. Each group dir has it extended attribute (keyed `xtrash_orig_path`) set to aid in undo.

## Known issues

As far as I understand, there is no good way to trash your files on macOS:

|Feature               	| Finder via AS  	| OS API¹  	| Direct   	| Comment                                        	|
|:-                    	|:--------------:	|:--------:	|:--------:	|                                                	|
| Speed                	| Slow           	| Fast     	|  Fast    	|                                                	|
| Sound                	| Beeps          	|          	|          	|                                                	|
| Requires Finder      	| ✗ yes          	| ✓ no     	| ✓ no     	| Extra startup delay if no Finder               	|
| Automation permission	| ✗ yes          	| ✓ no     	| ✓ no     	| asks for automation permissions on first run   	|
| Finds the right 🗑    	| ✓ yes          	| ✗ no     	| ✗ custom 	| "Root"-protected files are deleted to User's 🗑⁴ 	|
| Creates missing 🗑    	| ✓ yes          	| ✓ yes    	| ✗ custom 	| Might be tricky with setting proper permissions	|
| Undo                 	| "Put back"     	| ✗ custom²	| ✗ custom 	| "Put back" is a accessible via Finder          	|
| Undo data            	| `.DS_Store`³ @ 🗑	| custom   	| custom   	|                                                	|

¹ [FileManager](https://developer.apple.com/documentation/foundation/filemanager/)
² macOS bug existing for many years, though some reports that it works in the latest Sequoia version?
³ proprietary Apple format that is refreshed by Finder with a 2 second (?) delay on trash, so undo of the most recent files programmatically might fail
⁴ with elevated/sudo permissions files are deleted to root user's trash (`/private/var/root/.Trash` or `/Volumes/X/.Trashes/0`) instead of the logged in user's trash (`~/.Trash`)

This tool tries the "Direct" approach with a custom undo functionality implemented by saving the original paths in extended attributes instead of the `.DS_Store` file database. Though the alternative of using `.DS_Store` or a custom open database are also an option.

## Credits

- [trash101](https://github.com/russelldavis/trash101) Pyhon version introducing the extended attributes approach
