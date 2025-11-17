## Manifest File Format

### 1. packages.json

This is the top-level manifest that contains a list of available packages.

#### Structure:
```json
{
  "packages": [
    {
      "name": "example",
      "versions": [
        {
          "version": "v1.0.0",
          "url": "https://example.com/artifact.tar.gz",
          "checksum": "SHA256_CHECKSUM",
          "manifest": "manifest.json"
        }
      ]
    }
  ]
}

