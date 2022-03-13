# Toolchains & Stuff

A table of natives grouped together with their identifier
and [compatible Rust toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html)

| Identifier (`platform-architecture`) | Compatible Rust Toolchain          | Notes                                                                      |
|--------------------------------------|------------------------------------|----------------------------------------------------------------------------|
| `darwin`\*                           | `x86_64-apple-darwin`              |
| `darwin-x86-64`*\*                   | `x86_64-apple-darwin`              |
| `darwin-aarch64`*\*                  | `aarch64-apple-darwin`             |
| `linux-aarch32`                      | `armv7-unknown-linux-gnueabi`*\*\* |
| `linux-aarch64`                      | `aarch64-unknown-linux-gnu`*\*\*   |
| `linux-arm`                          | `arm-unknown-linux-gnueabi`*\*\*   |
| `linux-armhf`                        | `arm-unknown-linux-gnueabihf`*\*\* |
| ~~`linux-musl-x86-64`*\*~~           | ~~`x86_64-unknown-linux-musl`~~    | libsamplerate-sys doesn't have the correct build process for musl support. |
| `linux-x86-64`                       | `x86_64-unknown-linux-gnu`         |
| `linux-x86`                          | `i686-unknown-linux-gnu`           |
| `win-x86-64`                         | `x86_64-pc-windows-msvc`           |
| `win-x86`                            | `i686-pc-windows-msvc`             |

*only required for original lavaplayer

*\*not detected by lavaplayer

*\*\*no guarantee these will work
