# `gen-chbs`

Command-line tool to generate passphrases or funny names
based on [`chbs` crate][crate-chbs].

[crate-chbs]: https://crates.io/crates/chbs

## Why?

I needed to generate simple fun repository names,
but you can use it to generate easy to remember passwords.

## How to run

With `cargo`:

```bash
cargo run -- --help
```

With `nix`:

```bash
nix run .#gen-chbs -- --help
```

## License

[MIT](./LICENSE).
