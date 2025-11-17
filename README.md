# gip — Git Package Manager

**gip** is a personal, GitHub-backed package manager written in Rust.
It installs your own prebuilt binaries directly from GitHub Releases with checksum verification.

---

## Features
- Fetches manifests and binaries over HTTPS
- Verifies SHA256 integrity
- Installs to `~/.gip/` symlinked to `~/.local/bin/` with versioned paths
- Simple JSON-based manifest system
- Works on Linux and macOS

---

## Structure

```
packages/
├── mytool/
│  ├── v1.0.0/
│  │    ├── mytool-1.0.0-linux-amd64.tar.gz
│  │    └── manifest.json
│  └── latest -> v1.0.0
└── packages.json
```

Each `manifest.json` contains the version, file URLs, and SHA256 checksums.

---

## Usage

```bash
# install a package
gip install mytool

# update to latest
gip update mytool

# list installed packages
gip list

# remove a package
gip uninstall mytool
````

Binaries are placed in:

```
~/.gip/<name>-<version>
~/.local/bin/<name> -> symlink to current version
```

---

## Security

* Downloads verified with SHA256
* Optional GPG signature validation
* All traffic over HTTPS

---
