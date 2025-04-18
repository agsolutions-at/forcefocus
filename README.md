# forcefocus

`forcefocus` is a modern reimplementation of [robinwassen/forcefocus](https://github.com/robinwassen/forcefocus), featuring prebuilt bindings written
in Rust for enhanced performance and reliability. This project provides the same native logic and usage as the original `forcefocus` tool, enabling
you to programmatically bring a specific window to the foreground on Windows.

## Features

- Heavily inspired by the original `forcefocus` project.
- Prebuilt Rust bindings for improved performance and cross-platform compatibility.
- Easy-to-use Node.js interface for integrative purposes.
- No need to build from source when used in an Electron project anymore.

## Usage

This package provides a simple API to focus a window by its handle using Node.js. For example:

```typescript
import process from 'node:process';
import { BrowserWindow } from 'electron';

const isWindows = process.platform === 'win32'

async function bringWindowToFront(window: BrowserWindow) {
   if (isWindows) {
      const { focusWindow } = await import('@agsolutions-at/forcefocus')
      focusWindow(window.getNativeWindowHandle())
      return
   }

   // other platforms
   window.focus()
}
```

The primary function `focusWindow` takes a `Buffer` containing the window handle and brings the corresponding window to the foreground.
The lib works on Windows only, so please make sure you don't import it on other platforms.

### Installation

To add this package to your Node.js project, use a package manager of your choice:

```bash
# npm
npm install @agsolutions-at/forcefocus

# yarn
yarn add @agsolutions-at/forcefocus

# pnpm
pnpm add @agsolutions-at/forcefocus
```

### Building from Source

If you prefer to build this package from source, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/agsolutions-at/forcefocus.git
   cd forcefocus
   ```
2. Install dependencies:
   ```bash
   yarn install
   ```
3. Build the project:
   ```bash
   yarn build
   ```

## Contributing

Contributions are welcomed.
Please feel free to open issues or pull requests on the GitHub repository.

## License

The project is licensed under the MIT license.
