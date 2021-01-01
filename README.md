## Shell

-   **Bash**

Add the following line to your `~/.bashrc`

```bash
eval "$(snm eval bash)"
```

-   **Zsh**

Add the following line to your `~/.zshrc`

```zsh
eval "$(snm eval zsh)"
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
