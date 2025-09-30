# CLI Unit Testing Documentation

## Overview

This document describes the comprehensive unit testing suite added for CLI arguments in the DDNS Updater project. The tests validate the complete flow from CLI argument parsing down to specific actions.

## Test Structure

### 1. CLI Arguments Tests (`src/cli/args_test.rs`)

Tests the core CLI argument structure and validation:

- **test_args_creation_with_all_fields**: Validates all CLI arguments can be set and accessed correctly
- **test_args_creation_with_directory_config**: Tests directory-based configuration setup
- **test_args_flag_combinations**: Validates different combinations of verbose and no-reload flags
- **test_args_pattern_variations**: Tests various file pattern matching options
- **test_args_hostname_variations**: Validates different hostname formats
- **test_args_backup_directory_options**: Tests custom backup directory functionality
- **test_args_config_source_mutual_exclusivity**: Ensures single file and directory configs are mutually exclusive

### 2. Application Services Tests (`src/application/services_test.rs`)

Tests the application layer configuration and service factory:

- **test_app_config_creation**: Validates AppConfig structure creation
- **test_app_config_defaults**: Tests default configuration values
- **test_create_web_server_handler_***: Tests web server handler creation for different server types
- **test_create_*_service**: Tests creation of various services (network, notification, config discovery)
- **test_app_config_verbose_and_no_reload_combination**: Tests flag combinations in application config

### 3. Integration Tests (`tests/cli_integration_test.rs`)

Tests the complete CLI argument flow from input to action:

- **test_cli_args_single_file_integration**: Tests single config file processing
- **test_cli_args_directory_scan_integration**: Tests directory-based config discovery
- **test_cli_args_all_flags_integration**: Tests all CLI arguments working together
- **test_cli_args_error_handling_flow**: Tests error handling for invalid inputs
- **test_cli_args_pattern_and_hostname_combinations**: Tests various pattern/hostname combinations
- **test_cli_args_flag_combinations**: Tests different flag combinations

## CLI Arguments Covered

All 7 CLI arguments are thoroughly tested:

1. **--host**: Target hostname for IP resolution
2. **--config**: Single configuration file path
3. **--config-dir**: Configuration directory path
4. **--pattern**: File pattern matching (*.conf, *.nginx, etc.)
5. **--backup-dir**: Custom backup directory location
6. **--no-reload**: Skip server reload after configuration changes
7. **--verbose**: Enable verbose output

## Test Coverage

The tests cover:

- ✅ **Argument Parsing**: All CLI arguments can be parsed and accessed
- ✅ **Configuration Flow**: Arguments flow correctly through the application layers
- ✅ **Flag Combinations**: Various combinations of boolean flags work correctly
- ✅ **Pattern Matching**: Different file patterns are handled properly
- ✅ **Error Handling**: Invalid inputs produce appropriate errors
- ✅ **Service Creation**: All application services can be created successfully
- ✅ **Integration**: Complete end-to-end argument processing

## Running Tests

```bash
# Run all unit tests
cargo test --lib

# Run integration tests
cargo test

# Run specific CLI tests
cargo test cli

# Run application layer tests
cargo test application

# Run shell-based integration tests (validates actual CLI functionality)
./scripts/test_cli_simple.sh
```

## Test Architecture

The tests follow the clean architecture layers:

1. **Interface Layer**: CLI argument structure and validation
2. **Application Layer**: Service configuration and factory patterns
3. **Integration Layer**: End-to-end argument flow validation

Each layer is tested independently to ensure proper separation of concerns while also testing the complete integration flow.

## Key Testing Principles

1. **Unit Isolation**: Each test focuses on a specific aspect of CLI argument handling
2. **Integration Validation**: Tests verify the complete argument flow
3. **Error Coverage**: Both success and failure scenarios are tested
4. **Realistic Scenarios**: Tests use realistic hostnames, file patterns, and directory structures
5. **Architecture Compliance**: Tests respect the clean architecture boundaries

## Benefits

- **Regression Prevention**: Changes to CLI argument handling are immediately caught
- **Documentation**: Tests serve as living documentation of CLI behavior
- **Confidence**: Comprehensive coverage ensures CLI arguments work as expected
- **Maintainability**: Well-structured tests make future changes safer