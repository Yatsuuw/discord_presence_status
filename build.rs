//! Build script: embeds the icon and version information into the Windows executable.
//!
//! On other operating systems it does nothing.

/// Compiles the Windows resource file (icon, product name, version…).
///
/// # Errors
///
/// Fails the build if the resource compiler (part of the Windows SDK) is
/// missing or rejects the resource description.
fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=assets/icons/app.ico");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Ok(());
    }

    let mut resource = winresource::WindowsResource::new();
    resource
        .set_icon("assets/icons/app.ico")
        .set("ProductName", "Discord Presence Status")
        .set("FileDescription", "Discord Presence Status")
        .set("InternalName", "discord_presence_status")
        .set("OriginalFilename", "discord_presence_status.exe")
        .set(
            "LegalCopyright",
            "Copyright (c) 2026 Yatsuu. All rights reserved.",
        );
    resource.compile()
}
