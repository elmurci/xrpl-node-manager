# XRP Ledger Node Manager

[![Open in Remote - Containers](https://img.shields.io/static/v1?label=Remote%20-%20Containers&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/elmurci/xrpl-node-manager)

PoC. Work in progress...

User friendly tool to manage a XRPL Node.

## Run locally

There are two components: 

- `service`: backend interacting with the XRPL Node
- `ui`: Frontend

For now, you need to be running (or point to one with admin access) a `rippled` node. Configuration can be changed in the `service` (`config` folder)

### Service

`cd service && cargo run`

### UI

`cd ui && yarn dev`

## Demo

![alt text](xrplnodemanager.gif)

## TODO/Ideas

- Issue tokens (dev)
- AMM features
- Sidechains
- Wallet creation / Faucet (dev)
- Explorer
- ...
