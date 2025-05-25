// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

fn main() {
    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        winresource::WindowsResource::new()
            .set_manifest_file("src/bin/edit/edit.exe.manifest")
            .set_icon("assets/edit.ico")
            .set("ProductName", "Microsoft Edit")
            .set("FileDescription", "Microsoft Edit")
            .set("LegalCopyright", "Copyright (c) Microsoft Corp 2025")
            .set("OriginalFilename", "EDIT.COM")
            .compile()
            .unwrap();
    }
}
