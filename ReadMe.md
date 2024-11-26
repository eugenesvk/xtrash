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

  1. Ask `Finder` via AppleScript: slow, beeps, interrupts user asking for automation permissions, requires Finder running (which you might not have if you use an alternative file manager, so that's extra startup delay), but then is the only method that enables "put back"/undo feature via Finder
  2. Using Obj-c FileManager APIs: none of the Finder downsides, __but__ has an OS bug where "put back"/undo is broken for many years. This could be solved by manually editing `.DS_Store`, but I'm not aware of any tools that do that. Also, Finder is slow to flush changes to disk, so undo operations might skip the latest trashes that Finder has completed, but not recorded yet

So this tries a more direct approach

  3. Trash files by directly moving them to the trash folder and setting their extended attributes to the path they were deleted from.

## Credits

- [trash101](https://github.com/russelldavis/trash101) Pyhon version introducing the extended attributes approach
