# tgcrab

Unofficial Telegram client for the terminal.

> **Unofficial notice:** This is an unofficial, third-party project.
> It is not affiliated with, endorsed by, or sponsored by Telegram.
> It uses Telegram's official servers via TDLib, but it is not an
> official Telegram client.

## Features

- Terminal-first Telegram client built in Rust.
- Uses TDLib (via the `tdlib-rs` crate) to talk to Telegram's servers.
- XDG-compliant config and data directories.
- Tracing-based logging with a strict no-secrets policy.
- Opus of a clean workspace: `tgcrab-core` (logic, no UI) and
  `tgcrab-cli` (the `tgcrab` binary).

## Quickstart

```sh
cp .env.example .env   # then fill in TG_API_ID / TG_API_HASH from https://my.telegram.org
cargo build
cargo run -p tgcrab-cli -- --help
cargo run -p tgcrab-cli -- --version
```

## Status

Scaffold / pre-release. The workspace builds and the CLI responds to
`--help` / `--version`. Login, chats, history, and sending are planned
for later milestones.

## License

Dual-licensed under the MIT License and the Apache License, Version 2.0.
See `LICENSE-MIT` and `LICENSE-APACHE`.
