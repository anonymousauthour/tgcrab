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
- Clean workspace layout: `tgcrab-core` (logic, no UI) and
  `tgcrab-cli` (the `tgcrab` binary).

## Quickstart

```sh
cp .env.example .env   # then fill in TG_API_ID / TG_API_HASH from https://my.telegram.org
cargo build
cargo run -p tgcrab-cli -- --help
cargo run -p tgcrab-cli -- --version
```

## Status

Bootstrap complete (Phase 6 of the initial plan): workspace builds,
TDLib smoke test passes (`cargo test`), and the CLI responds to
`--help` / `--version`. Login, chats, history, and sending are planned
for later milestones.

## Milestone 1 plan: login flow

- Phone-number login: prompt for phone, request code, accept the code
  from stdin, optional 2FA password.
- Optional QR login via `tdlib-rs` updates (`qrCode` / `updateLoginWithQrCode`).
- Session persists through TDLib's encrypted database (key in the OS
  keyring, fallback `TGCRAB_DB_KEY`).
- CLI commands `chats`, `history`, `send`, `me`, `logout` arrive in
  milestone 2.

## License

Dual-licensed under the MIT License and the Apache License, Version 2.0.
See `LICENSE-MIT` and `LICENSE-APACHE`.
