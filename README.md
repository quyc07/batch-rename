# Batch Rename

The `batch_rename` command line tool allows you to rename multiple files at once according to your chosen naming convention.

## Usage

To use `batch_rename`, run the command followed by the desired options:

```bash
batch_rename [OPTIONS]
```

### Options

- `-path <path>`: The path to the directory containing the files to rename.
- `-prefix <prefix>`: The prefix to add to the file names.
- `-postfix <postfix>`: The postfix to add to the file names.
- `-replace <old> <new>`: Replace the old string in the file names with the new string.
- `-new <name>`: Use a new name for the files, with a number appended to each file name.
- `-help`: Print the help message.

## Examples

- `batch_rename --path /path/to/directory --prefix "new_"`: Add the prefix "new_" to all file names in the specified directory.
- `batch_rename --path /path/to/directory --replace "old" "new"`: Replace the string "old" with "new" in all file names in the specified directory.