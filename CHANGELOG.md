# Changelog

## v0.9.0

* Improved quality of errors
* More commands return an error when the given entry doesn't exist
* Fixed bug where `Entry does not exist` error would happen when the entry definitely existed
* All commands now case-insensitive
* Slight optimisations made

## v0.8.0

* Added `copy` flag to `show` to allow for
copying a password to the clipboard
* Added `interactive` flag to `add` command

## v0.7.0

* Added ability to change default file path

## v0.6.5

* Fixed `No such file or directory` error

## v0.6.4

* `show` command now shows the number of notes for an entry

## v0.6.3

* Changed some arguments to `notes` commands to use positional arguments rather than flags
where appropriate
* Changed formatting of `notes ls` to show the note id in square brackets
