## Rust compiler working in vscode

1.) Delete everything in ~/.rustup/ (which removes toolchains, defaults, components and overrides)

`$ rm -rf ~/.rustup/`

2.) Install toolchain, set default, install components

```$ rustup install stable
$ rustup default stable
$ rustup component add rls rust-analysis rust-src
```

3.) Uninstall Rust (rls) plugin in VSCode, restart VSCode, install the plugin, close VSCode
4.) Run `cargo check` && `cargo build` && `cargo doc` on respository
5.) Open VSCode again -> RLS is building
