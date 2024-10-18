# Forge Migrator Workspace

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

`Forge Migrator` is a rust workspace that provides a suite of tools for managing Laravel Forge migrations. This repository contains three packages: `forge_common`, `forge_migrate` and `forge_reset`. Together these tools allow users to easily migrate and reset LAravel Forge servers using a highly customizable and reliable Rust-based CLI.

## Workspace Overview

This workspace contains the following crates:

### 1. [forge_common](./forge_common)
`forge_common` contains shared utilities and functions used by both `forge_migrate` and `forge_reset`. It ensures consistency across the tools and provides reusable components, such as API handling and server communication logic.

### 2. [forge_migrate](./forge_migrate)
`forge_migrate` is the core migration tool that automates the process of moving Laravel Forge-managed sites between servers. It leverages `forge_common` for backend operations and provides an easy-to-use command-line interface to perform migrations securely.

#### Key Features:
- Automated migration of Laravel Forge sites between servers.
- Use secure communication with Forge's API.

### 3. [forge_reset](./forge_reset)
`forge_reset` will undo the migration as per the configuration, it will delete the site, database and user on the destination server. 

## Installation

You can install the `forge_migrate` tool via `cargo`:

```bash
cargo install forge_migrate
```




