# Automated Release System

This project includes an automated release system that creates GitHub releases whenever you push a version tag.

## How It Works

### 🏷️ Tag-Based Releases
When you push a git tag following the pattern `v*.*.*` (e.g., `v1.2.0`), the automated release workflow will:

1. **Extract version information** from the git tag
2. **Build binaries** for both x86_64 (amd64) and ARM64 architectures  
3. **Generate archives** (.tar.gz files) for each architecture
4. **Create checksums** (SHA256SUMS) for verification
5. **Extract changelog** content for the specific version
6. **Create GitHub release** with all assets and documentation

### 📁 Assets Created

Each release automatically includes:
- `ddns_updater-vX.X.X-amd64` - Direct x86_64 binary
- `ddns_updater-vX.X.X-arm64` - Direct ARM64 binary  
- `ddns_updater-vX.X.X-amd64.tar.gz` - x86_64 archive
- `ddns_updater-vX.X.X-arm64.tar.gz` - ARM64 archive
- `SHA256SUMS` - Checksums for verification

## Creating a Release

### Step 1: Update Version
```bash
# Update Cargo.toml version
sed -i 's/version = "1.0.0"/version = "1.1.0"/' Cargo.toml

# Optional: Update CHANGELOG.md with new version section
```

### Step 2: Commit and Tag
```bash
# Commit version changes
git add Cargo.toml CHANGELOG.md
git commit -m "Release v1.1.0: Description of changes"
git push

# Create and push tag
git tag -a v1.1.0 -m "Release v1.1.0: Description of changes" 
git push origin v1.1.0
```

### Step 3: Automatic Release
The automated release workflow will:
- ✅ Trigger automatically when the tag is pushed
- ✅ Build both x86_64 and ARM64 binaries
- ✅ Verify Debian 12 compatibility (glibc ≤ 2.36)
- ✅ Create GitHub release with installation instructions
- ✅ Upload all assets and checksums

## Workflow Details

### 🔧 Build Environment
- **Runner**: Ubuntu 22.04
- **Rust Version**: 1.82.0 (consistent across all workflows)
- **Targets**: x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu
- **Cross-compilation**: Native GCC toolchain for ARM64

### 📋 Compatibility
- **Debian 12+** (glibc 2.34+)
- **Ubuntu 22.04+**
- **Architecture**: x86_64 (Intel/AMD) and ARM64 (Apple Silicon, ARM servers)

### 🚀 Speed Optimizations
- Uses GitHub's ubuntu-22.04 runners (high availability)
- Builds both architectures in parallel steps
- Minimal dependencies for faster setup
- Efficient binary packaging

## Changelog Integration

If a `CHANGELOG.md` file exists, the workflow will:
1. **Extract the section** for the current version (e.g., `## [1.1.0]`)
2. **Include it** in the GitHub release description
3. **Fall back** to a generic message if no changelog section is found

### Changelog Format
```markdown
# Changelog

## [1.1.0] - 2025-10-02
### Added
- New email notification system
- CLI email configuration flags

### Changed  
- Upgraded to Rust 1.82.0
- Enhanced Debian 12 compatibility

### Fixed
- Cross-compilation issues
- GLIBC version requirements
```

## Manual Override

If you need to create a release manually, you can still use the existing manual release workflow:

```bash
# Go to GitHub → Actions → "Release Management" → "Run workflow"
# Enter version: v1.1.0
# Click "Run workflow"
```

## Troubleshooting

### Tag Already Exists
```bash
# Delete local and remote tag
git tag -d v1.1.0
git push --delete origin v1.1.0

# Recreate and push
git tag -a v1.1.0 -m "Release v1.1.0: Updated description"
git push origin v1.1.0
```

### Build Failures
- Check the **Actions** tab for detailed error logs
- Verify Rust 1.82.0 compatibility of dependencies
- Ensure `Cargo.toml` version matches the git tag version

### Missing Assets
- The workflow creates binaries for both x86_64 and ARM64
- Check the workflow logs if any architecture fails to build
- Verify cross-compilation toolchain installation

## Security

- Uses `GITHUB_TOKEN` with minimal required permissions (`contents: write`)
- No external secrets or credentials required
- All builds run in isolated GitHub Actions runners
- Checksums provided for asset verification