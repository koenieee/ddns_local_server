# Test Configuration Files

This directory contains test configuration files used for unit testing the nginx config validation functionality.

## Structure

### `valid/` - Valid Nginx Configuration Files
- `basic_server.conf` - Simple server block with basic directives
- `complex_ssl.conf` - Complex configuration with SSL, upstream, and proxy settings
- `full_nginx.conf` - Complete nginx configuration with events and http blocks
- `minimal_valid.conf` - Minimal but valid nginx configuration

### `invalid/` - Invalid Configuration Files
- `plain_text.conf` - Plain text file with no nginx directives
- `no_braces.conf` - Has nginx directives but missing proper brace structure
- `only_comments.conf` - Contains only comments, no actual configuration
- `insufficient_structure.conf` - Has minimal nginx content but insufficient structure
- `json_file.conf` - JSON content (has braces but no nginx directives)
- `empty_file.conf` - Completely empty file

## Usage

These files are used by the unit tests in `src/config/nginx.rs` to verify that:

1. Valid nginx configurations are properly identified and accepted
2. Invalid configurations are properly rejected
3. Edge cases are handled correctly
4. The validation logic works with various nginx directive patterns

## Running Tests

```bash
# Run all tests
cargo test

# Run only nginx validation tests
cargo test config::nginx::tests

# Run tests with output
cargo test -- --nocapture
```

## CLI Testing

You can also test the CLI functionality with these directories:

```bash
# Test with valid configs
cargo run -- --config-dir test_configs/valid --verbose

# Test with invalid configs (should fail)
cargo run -- --config-dir test_configs/invalid --verbose
```