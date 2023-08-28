# Bridge Controller

The front-end part of the MPC interface setup that interconnects common cryptographic interfaces with [Meesign](https://meesign.crocs.fi.muni.cz/).

## Currently Supported Interfaces

- [PKCS#11 (Cryptoki)](https://github.com/KristianMika/cryptoki-bridge)
- [FIDO](https://github.com/KristianMika/softfido)

## Development

### Build Requirements

- [tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/)
- [npm, Node.js](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [rust](https://www.rust-lang.org/tools/install)
- [protocol buffer compiler](https://grpc.io/docs/protoc-installation/)

### Dev Container

The [devcontainer](./.devcontainer) folder contains a configuration of a development Docker environment.

_Please, note, this setup has some limitations. For example, you can upload files only from the `/tmp/bridge-files` directory (must be created manually before running the setup, can be changed in [devcontainer.json](./.devcontainer/devcontainer.json) under `mounts`._

1. Install the `ms-vscode-remote.remote-containers` VS Code extension.
2. Create a shared directory `/tmp/bridge-files/` (_important, don't skip_)
3. Press `Ctrl + Shift + P`, select `>Dev Containers: Open folder in Container...`, and select the root repository directory. (_this may take some time for the the first run_)

### Development Build

- Launch a development server. The setup watches all files and reflects the changes on the fly
  ```bash
  npm run tauri dev
  ```

## Production Build

- Create a production build
  ```bash
  npm run tauri build
  ```
