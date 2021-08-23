<div align="center">

![frum](./resources/logo.png)

A little bit fast and modern Ruby version manager written in Rust

[![github workflow status](https://img.shields.io/github/workflow/status/TaKO8Ki/frum/CI/main)](https://github.com/TaKO8Ki/frum/actions) [![crates](https://img.shields.io/crates/v/frum.svg?logo=rust)](https://crates.io/crates/frum) [![brew](https://img.shields.io/homebrew/v/frum?color=blue)](https://formulae.brew.sh/formula/frum) [![aur](https://img.shields.io/aur/version/frum-bin?color=yellow)](https://aur.archlinux.org/packages/frum-bin)

![usage](./resources/frum.gif)

</div>

## Features

- Pure Rust implementation not using `ruby-build`
- Cross-platform support (macOS, Linux)
- Works with `.ruby-version` files
- Auto-Completion

## Goals

- **Blazing-Fast Ruby Installation** - built with speed in mind
- **Cross-Platform** - works on macOS, Linux and (Windows)

### Benchmark

`eval "$(frum init)"` runs about 6 times faster than `eval "$(rbenv init -)"`.

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `eval "$(rbenv init -)"` | 49.5 ± 2.1 | 46.2 | 57.2 | 6.14 ± 0.50 |
| `eval "$(frum init)"` | 8.1 ± 0.7 | 7.0 | 11.8 | 1.00 ± 0.11 |
| `eval "$(frum init)"` (pre-release) | 8.1 ± 0.6 | 7.2 | 11.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `rbenv` | 239628.1 ± 2030.2 | 237681.6 | 245162.6 | 1.04 ± 0.01 |
| `frum` | 232944.6 ± 1224.0 | 230565.4 | 234863.5 | 1.01 ± 0.01 |
| `frum` (pre-release) | 230366.5 ± 882.7 | 228454.2 | 232340.5 | 1.00 |

For more information, please see [#16](https://github.com/TaKO8Ki/frum/pull/16).

## Installation

### Homebrew (Linux/macOS)

If you’re using Homebrew or Linuxbrew, install the [`frum`](https://formulae.brew.sh/formula/frum) formula. For more information, please see [Install Ruby with Frum](https://mac.install.guide/ruby/14.html) written by Daniel Kehoe.

```
$ brew install frum
```

### Arch Linux

If you’re using Arch Linux, install the [`frum-bin`](https://aur.archlinux.org/packages/frum-bin) package using your favorite AUR helper.

```
$ yay -S frum-bin
```

### Cargo (Linux/macOS)

If you already have a Rust environment set up, you can use the `cargo install` command:

```
$ cargo install frum
```

### Using a release binary (Linux/macOS)

- Download the [latest release binary](https://github.com/TaKO8Ki/frum/releases) for your system
- Set the `PATH` environment variable
- Configure your shell profile

## Usage

### Shell Setup

You need to run some shell commands before using frum. All you have to do is evaluate the output of `frum init`. Check out the following guides for the shell you use:

#### Bash

add the following to your `.bashrc`:

```bash
eval "$(frum init)"
```

#### Zsh

add the following to your `.zshrc`:

```zsh
eval "$(frum init)"
```

#### Fish shell

create `~/.config/fish/conf.d/frum.fish` add this line to it:

```fish
frum init | source
```

### Options

- **--log-level**: The log level of frum commands [default: info] [possible values: quiet, info, error].
- **--ruby-build-mirror**: [default: https://cache.ruby-lang.org/pub/ruby].
- **--frum-dir**: The root directory of frum installations [default: $HOME/.frum]. You can set `frum-dir` as the `$FRUM_DIR` environment variable. I recommend that you use the environment variable if you want to use your customized `frum-dir` globally.

### Subcommands

- **init**: Sets environment variables for initializing frum.
- **install**: Installs the specified Ruby version.
    - **-l**, **--list**: Lists the Ruby versions available to install.
- **uninstall**: Uninstall a specific Ruby version.
- **versions**: Lists installed Ruby versions.
- **global**: Sets the global Ruby version.
- **local**: Sets the current Ruby version.
- **clean**: Remove old downloads.

### Ruby configuration options

Options to configure Ruby can be passed to the `frum install` command.

```sh
$ frum install --with-openssl-dir=<ssl_dir> # Specify the OpenSSL directory
$ frum install --with-jemalloc # Use jemalloc as allocator
```

You can also specify many other options that will be listed when running `./configure -h`.

## Contribution

Contributions, issues and pull requests are welcome!

## Reference

- [Schniz/fnm](https://github.com/Schniz/fnm)
