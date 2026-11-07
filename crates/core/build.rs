//! Build script: links the prebuilt TDLib (`libtdjson`) shipped by the
//! `tdlib-rs` crate with the `download-tdlib` feature.
//!
//! At build time `tdlib-rs` downloads the prebuilt TDLib archive from its
//! GitHub releases and extracts it under `$OUT_DIR/tdlib`. We only need to
//! emit the link flags, which [`tdlib_rs::build::build_download_tdlib`]
//! does for us.

fn main() {
    tdlib_rs::build::build_download_tdlib(None);
}
