<h1 align="center">
    <code>🤏 Smol Node Manager</code>
</h1>

## Features

-   It's fast as it is written in Rust.
-   Cross Platfrom (Linux/macOS/Windows)
-   Supports `.nvmrc`, `.node-version` and `package.json`
-   Supports multiple shells
-   Tons of commands and options

## Installation

### From script (Linux/macOS/Windows)

```sh
curl -fsSL https://git.io/JLFnA | bash
```

#### Available Params

-   `--install-dir` : Set a custom directory for binary installation. Defaults to `$HOME/.snm`
-   `--skip-shell` : Skip appending shell specific loader to the `$SHELL` config file.

### From package managers

-   **Using `cargo` (Linux/macOS/Windows)**

```sh
cargo install snm
```

<!-- -   **Using `brew` (MacOs)** -->
<!--  -->
<!-- ```sh -->
<!-- brew install fnm -->
<!-- ``` -->

<!-- -   **Using `scoop` (Windows)** -->
<!--  -->
<!-- ```sh -->
<!-- scoop install fnm -->
<!-- ``` -->

-   **Using `yay` or `pamac` (Arch Linux)**

> Why snm is available for Arch? Because I love Arch Linux

```sh
# Using `yay`
yay -S snm

# Using `pamac`
pamac build snm
```

#### From binaries

Check out the [Release page](https://github.com/numToStr/snm/releases) for prebuild binaries for `snm`, available for different operating systems.

---

NOTE: `snm` uses symlinks underneath to manage aliases. So, If you are using **Windows** make sure you have enabled **Developer Mode** or your user has permission to create symlinks. You can read more [here](https://blogs.windows.com/windowsdeveloper/2016/12/02/symlinks-windows-10/)

---

## Shell

-   **Bash**

Add the following line to your `~/.bashrc`

```bash
eval "$(snm env bash)"
```

-   **Zsh**

Add the following line to your `~/.zshrc`

```zsh
eval "$(snm env zsh)"
```

-   **Fish**

Add the following line to your `~/.config/fish/config.fish`

```fish
snm env fish | source
```

-   **PowerShell**

Add the following line to your `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1`

```bash
snm env pwsh | Out-String | Invoke-Expression
```

You can find more about the powershell profile [here](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_profiles)

## Usage

### Global Options/Flags

| Options              | Env Variable           | Description                                         |
| -------------------- | ---------------------- | --------------------------------------------------- |
| `--snm-dir`          | `SNM_DIR`              | Directory where the all files and aliases are saved |
| `--node-dist-mirror` | `SNM_NODE_DIST_MIRROR` | Nodejs download mirror                              |

Example:

```sh
snm [--snm-dir="~/.something/else"] [--node-dist-mirror="https://myrelease.com"]
```

| Flags             | Description                |
| ----------------- | -------------------------- |
| `--download-only` | Only downloads the release |
| `--version`       | Prints the version         |
| `--help`          | Prints the help doc        |

### Commands

-   `snm install <version|alias>` : Install Nodejs with the provided version or lts codename

```sh
# Following command will downloads and installs the most recent 14.x.x release
snm install 14

# Following command will download the most recent lts/fermium release
snm install lts/fermium
# or snm install lts-fermium
# or snm i lts-fermium
```

-   `snm uninstall [version|alias]` : Removes the installed Nodejs

> If given an alias like `ten` or `lts-fermium` then it will remove the version which the alias is pointing at and all the aliases which are pointing to the same version.
> Also, uninstalling a version will throw an error, if multiple installation is found in the same semver range

```sh
# Following command will remove 14.x.x installation
snm uninstall 14

# Following command will download the most recent lts/fermium release
snm uninstall lts/fermium
# or snm uninstall lts-fermium
# or snm rm lts-fermium
```

-   `snm use [version]` : Change Nodejs version, Supports `.nvmrc` and `.node-version`

```sh
# Following command will use a downloaded version matching 10.x.x
snm use 10

# Searches for `.nvmrc` or `.node-version`, if <version> is not provided
snm use
```

-   `snm lts` : Installs the recent **lts** release

-   `snm latest` : Installs the recent **current** release

-   `snm ls` : List all the local downloaded versions with their aliases

-   `snm ls-remote [version]` : List remote Node.js versions

```sh
# Following command list 20 results with version matching 14.x.x
snm ls-remote 14

# This will show all the results
snm ls-remote 14 --all

# Following command will show 25 results
snm ls-remote 14 --count 25
```

-   `snm alias <version> <name>` : Alias a version to a common name

```sh
# Following command will alias the version 10 to ten
# 10 can refer to any semver release ie 10.15.0
snm alias 10 ten
```

-   `snm unalias [name]` : Removes aliases

```sh
# Following command will removes alias `ten`
snm unalias ten

# Removes all the aliases
snm unalias --all
```

-   `snm exec <version>` : Executes a command within snm context with the modified PATH

```sh
# Following command will output the Nodejs version
snm exec 10 -- node -v

# Following command will run yarn with Nodejs v10.x.x
snm exec 10 -- yarn start
```

-   `snm prune` : Remove all downloaded versions except the installed version

-   `snm which <version>` : Prints path for the downloaded Nodejs version

-   `snm help <subcommand>` or `snm <subcommand> --help` : Help doc for the subcommand

**NOTE: This is a small part of help doc. Please make sure to read the inbuilt help**

### Completions

`snm` binary has inbuilt completions supports. Please follow the instruction to generate completions according to your shell.

-   For Zsh

```sh
snm completions zsh
```

-   For Bash

```sh
snm completions bash
```

-   For Fish

```sh
snm completions fish
```

-   For PowerShell

```sh
snm completions pwsh
```

After generating the completions, please follow your shell instructions on how to load and use them.

## Contributing

PRs are always welcome. You can help me by adding more tests 🤞.

First, You need to install rust toolchain via [rustup](https://rustup.rs/).

-   **Setup**

```sh
git clone https://github.com/numtostr/snm

cd ./snm
```

-   **Build**

```sh
# For debug build
cargo build


# For release build
cargo build --release
```

-   **Running**

```sh
# Same as running `snm --help`
cargo run -- --help

# Same as running `snm lsr 14`
cargo run -- lsr 14
```

## Credits

This project would not be possible without these awesome projects.

-   [n](https://github.com/tj/n) for cli design
-   [fnm](https://github.com/Schniz/fnm) for giving me a base project and some code :)
