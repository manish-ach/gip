# **Architecture Documentation**

## Overview

This document outlines the architecture of the **`gip`** package manager.

### Components

**Client (Rust CLI)**
- Responsible for:
  - Downloading manifests.
  - Verifying checksums.
  - Installing binaries.

**Server (GitHub)**
- Hosts:
  - Packages and manifests.
  - Supports HTTP-based retrieval for clients.

**Local State**
- Stores:
  - Information about installed packages and their versions.
  - Updated after each installation or uninstallation.

### Data Flow

1. Client requests **`packages.json`** from the server.
2. It resolves the package name and version requested.
3. Fetches **`manifest.json`** for detailed package information.
4. Verifies the checksum of the downloaded artifact.
5. Extracts necessary files and installs them to a specified location.
6. Updates the local state to record the installation of the new package.

### Sequence Diagram

```plaintext
Client -> Server: Request packages.json
Server -> Client: Respond with packages.json
Client -> Server: Request manifest.json for specific package
Server -> Client: Respond with manifest.json
Client -> Server: Download package artifact
Client -> Local State: Update installed packages
