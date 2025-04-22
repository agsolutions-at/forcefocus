# forcefocus

[![npm version](https://img.shields.io/npm/v/@agsolutions-at/forcefocus.svg)](https://www.npmjs.com/package/@agsolutions-at/forcefocus)
[![npm downloads](https://img.shields.io/npm/dm/@agsolutions-at/forcefocus.svg)](https://www.npmjs.com/package/@agsolutions-at/forcefocus)
[![license](https://img.shields.io/npm/l/@agsolutions-at/forcefocus.svg)](./LICENSE)
[![node](https://img.shields.io/node/v/@agsolutions-at/forcefocus)](https://nodejs.org)
[![platforms](https://img.shields.io/badge/platforms-Windows-blue)](#)
[![CI](https://github.com/agsolutions-at/forcefocus/actions/workflows/CI.yml/badge.svg)](https://github.com/agsolutions-at/forcefocus/actions/workflows/CI.yml)

`forcefocus` is a modern reimplementation of [robinwassen/forcefocus](https://github.com/robinwassen/forcefocus), featuring prebuilt bindings written
in Rust for enhanced performance and reliability. This project provides the same native logic and usage as the original `forcefocus` tool, enabling
you to programmatically bring a specific window to the foreground on Windows.

## Features

- Heavily inspired by the original `forcefocus` project.
- Prebuilt Rust bindings.
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

## Using `electron-builder` to bundle the platform-specific native module

When building an Electron app, you may need native modules that are specifically compiled for your target operating system and architecture (like
getting the right key to fit the right lock). This script helps make sure the correct .node binaries are downloaded just before packaging the app
using electron-builder.

```json file=./package.json
{
  ...
  "build": {
    "beforePack": "./beforePack.js",
    "files": [
      ...
      "!**/node_modules/@agsolutions-at/forcefocus-*/**"
    ]
  },
  ...
}
```

Define a `beforePack` hook. Do not include optional dependencies of your build platform.

```typescript file=./beforePack.js
import path from 'node:path';
import https from 'node:https';
import fs from 'node:fs';
import forceFocusPackage from './app/node_modules/@agsolutions-at/forcefocus/package.json' with { type: 'json' };
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const downloadFile = (url, dest, cb) => {
   https
   .get(url, res => {
      // If redirect
      if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
         return downloadFile(res.headers.location, dest, cb); // follow redirect
      }

      const fileStream = fs.createWriteStream(dest);
      res.pipe(fileStream);

      fileStream.on('finish', () => {
         fileStream.close(cb); // call callback on finish
      });

      res.on('error', err => {
         fs.unlink(dest, () => {});
         cb(err.message);
      });
   })
   .on('error', err => {
      cb(err.message);
   });
};

const beforePack = context => {
   const { electronPlatformName, arch } = context;

   let archName;
   switch (arch) {
      case 0:
         archName = 'ia32';
         break;
      case 1:
         archName = 'x64';
         break;
      case 2:
         archName = 'armv7l';
         break;
      case 3:
         archName = 'arm64';
         break;
      case 4:
         archName = 'universal';
         break;
      default:
         throw Error('Unknown arch');
   }

   if (electronPlatformName === 'win32') {
      const downloadUrl = `https://github.com/agsolutions-at/forcefocus/releases/download/v${forceFocusPackage.version}/forcefocus.win32-${archName}-msvc.node`;
      const nativeModulePath = path.join(
              __dirname,
              'app',
              'node_modules',
              '@agsolutions-at',
              'forcefocus',
              `forcefocus.win32-${archName}-msvc.node`
      );
      downloadFile(downloadUrl, nativeModulePath, err => {
         if (err) {
            console.error('Download error:', err);
            process.exit(1);
         } else {
            console.log('Download forcefocus completed');
         }
      });
   }
};

export default beforePack;
```

`beforePack.js`: before your app gets packaged, it sneaks in and places the correct native modules on stage based on the OS and architecture you're
targeting.
It detects whether you're building for Windows or macOS, figures out the architecture (x64, arm64, etc.), downloads the correct version of native
.node files from GitHub releases and saves them into the appropriate module folders.


## Contributing

Contributions are welcomed.
Please feel free to open issues or pull requests on the GitHub repository.

## License

The project is licensed under the MIT license.
