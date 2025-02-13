# Source code of the installer for TMA

## Building
- Install [Rust](https://www.rust-lang.org/) and [NodeJS](https://nodejs.org/)
- Install pnpm globally `npm i -g pnpm`
- Run `pnpm install` to install dependencies
- Run `pnpm tauri add updater`
- Go to `src-tauri/src/lib.rs` and remove this line `.plugin(tauri_plugin_updater::Builder::new().build())`
- You can then run `pnpm tauri dev` or `pnpm tauri build`

## License

This project is licensed under the **Restricted Project License (RPL) v1.0**.  

You may use, modify, and distribute this software **only for the purpose of building and distributing the original project**.  

**You are not allowed** to use any part of this software in other projects, whether personal or commercial.  

For more details, see the [LICENSE](./.github/LICENSE) file.
