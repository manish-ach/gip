# **Project Plan — Personal Package Manager**

## 1. Overview

**Name:** `gip`
**Purpose:** Install and manage my personal Rust-built tools and important binaries from GitHub Repo.
**Goals:**
* Simple HTTP-based install and update.
* Checksum and optional signature verification.
* Cross-platform (Linux, macOS).

---

## 2. Objectives

1. Automate installation of binaries from GitHub Releases.
2. Maintain local version tracking and rollback.
3. Use minimal external dependencies.
4. Ensure integrity and security with SHA256 verification.

---

## 3. System Architecture

### Components

| Component             | Description                                                  |
| --------------------- | ------------------------------------------------------------ |
| **Client (Rust CLI)** | Downloads manifests, verifies, installs binaries.            |
| **Server (GitHub)**   | Hosts artifacts and manifests.                               |
| **Manifests**         | JSON files defining versions, URLs, and checksums.           |
| **Local State**       | JSON or SQLite file storing installed packages and versions. |

### Data Flow

1. Client requests `packages.json`.
2. Resolves package and version.
3. Fetches `manifest.json`.
4. Verifies checksum/signature.
5. Extracts and installs binary.
6. Updates local state file.

---

## 4. Directory Structure

```
/project-root
 ├── src/
 │   ├── main.rs
 │   ├── client.rs
 │   ├── manifest.rs
 │   ├── install.rs
 │   └── verify.rs
 ├── docs/
 │   ├── plan.md
 │   ├── api.md
 │   └── architecture.md
 ├── packages/
 │   ├── example/
 │   │    └── v1.0.0/
 │   │        ├── manifest.json
 │   │        └── binary.tar.gz
 │   └── packages.json
 ├── tests/
 ├── Cargo.toml
 └── README.md
```

---

## 5. Functional Requirements

| ID | Requirement                                          | Priority |
| -- | ---------------------------------------------------- | -------- |
| F1 | Fetch and parse top-level manifest (`packages.json`) | High     |
| F2 | Download package artifact                            | High     |
| F3 | Verify SHA256 checksum                               | High     |
| F4 | Install binary to fixed location (`~/.local/bin`)    | High     |
| F5 | Maintain local installation state                    | Medium   |
| F6 | Support rollback to previous version                 | Medium   |
| F7 | Verify GPG signature (optional)                      | Low      |

---

## 6. Technical Stack

* **Language:** Rust
* **Libraries:**

  * `tokio`, `reqwest`, `serde`, `serde_json`, `sha2`, `tar`, `flate2`, `semver`
* **Build system:** Cargo
* **CI/CD:** GitHub Actions (matrix build)
* **Hosting:** GitHub Releases and Pages

---

## 7. Security Model

* All artifacts downloaded via HTTPS.
* SHA256 integrity check required.
* Optional GPG signature verification.
* No elevated privileges; installs user-local.

---

## 8. Development Roadmap

| Phase | Deliverable                      | Description                           | Duration |
| ----- | -------------------------------- | ------------------------------------- | -------- |
| 1     | Prototype manifest reader        | Parse JSON manifests and print info   | 1 week   |
| 2     | Downloader and checksum verifier | Fetch and verify artifact             | 1 week   |
| 3     | Installer                        | Extract and symlink binaries          | 1 week   |
| 4     | Local state tracker              | Maintain installed packages           | 1 week   |
| 5     | CI/CD pipeline                   | Build and publish with GitHub Actions | 1 week   |
| 6     | Documentation                    | Write README and CLI usage            | 2 days   |

*note: the duration is not strict and may be subject to discrepencies*

---

## 9. Documentation Checklist

* [ ] `README.md` (usage + install)
* [ ] `ARCHITECTURE.md` (data flow + component diagram)
* [ ] `API.md` (manifest format)
* [ ] `CONTRIBUTING.md` (if public)
* [ ] `SECURITY.md` (checksum, signatures)

---

## 10. Maintenance Plan

* Versioned releases (`vX.Y.Z`).
* Auto-update manifest in each release.
* Manual trigger for rebuilds.
* Log file of install/uninstall actions.

---
