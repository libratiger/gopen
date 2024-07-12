# gopen

[中文版本](README_zh.md)

`gopen` is a command-line tool designed to open the remote URL of a Git repository in your browser like macOS `open` command.

## Features

- **Automatic Remote URL Opening**: Extracts the remote URL from the Git repository in the current or specified directory and opens it in the browser.
- **SSH to HTTPS Conversion**: Automatically converts SSH URLs to HTTPS URLs for browser access.
- **Interactive Mode**: Allows users to select a remote URL from multiple options through an interactive interface.
- **Directory Specification**: Users can specify a directory to search for and open the remote URL from that directory.

## Installation

You can install `gopen` via Homebrew:

```sh
brew tap libratiger/homebrew-tap
brew install gopen
```

## Usage

### Default Mode

Run `gopen` in the current Git repository directory to automatically open the first remote URL.

```sh
gopen
```

### Specified Directory

Search for and open the remote URL in the specified directory.

```sh
gopen /path/to/git-repo
```

### Interactive Mode

Use the `-i` parameter to enter interactive mode, allowing the user to select a remote URL to open.

```sh
gopen -i
```

Or use interactive mode in a specified directory:

```sh
gopen /path/to/git-repo -i
```

## Examples

```sh
# Default mode, opens the remote URL in the current directory
gopen

# Specified directory
gopen /path/to/git-repo

# Interactive mode
gopen -i

# Specified directory with interactive mode
gopen /path/to/git-repo -i
```

## Contributing

Issues, feature requests, and pull requests are welcome. Please visit the [GitHub repository](https://github.com/libratiger/gopen) to contribute.

## License

`gopen` is licensed under the MIT License. 
