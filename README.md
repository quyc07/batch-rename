batch_rename(br)
----------------

A command line tool for batch renaming files in a directory.

## Documentation quick links

* [Installtion](#installtion)
* [Usage](#usage)
* [Options](#options)
* [Example](#example)

## Installtion

To install this tool using cargo, run the following command in your terminal:

```shell
cargo install batch_rename
```

Make sure you have Rust and Cargo installed on your system before running this command.

## Usage

To use `batch_rename`, run the command followed by the desired options:

```shell
br [OPTIONS]
```

## Options

- `--path <path>`: The path to the directory containing the files to rename.
- `--prefix <prefix>`: The prefix to add to the file names.
- `--postfix <postfix>`: The postfix to add to the file names.
- `--replace <old> <new>`: Replace the old string in the file names with the new string.
- `--new <name>`: Use a new name for the files, with a number appended to each file name.
- `--help`: Print this help message.

## Example

Add a prefix "new_" to all files in the directory "/path/to/directory":

```
br --path /path/to/directory --prefix new_
```

Replace all occurrences of "old" with "new" in the file names of the directory "/path/to/directory":

```
br --path /path/to/directory --replace old new
```

Use a new name "file" for all files in the directory "/path/to/directory", with a number appended to each file name:

```
br --path /path/to/directory --new file
