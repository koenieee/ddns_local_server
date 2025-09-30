# DDNS Updater - Test Scripts

This directory contains all the shell scripts for testing the DDNS Updater project.

## Scripts

### `test_cli_simple.sh`
Simple and reliable CLI integration tests.
```bash
./test_cli_simple.sh
```

### `test_all.sh`
Comprehensive test suite including unit tests, CLI tests, and quality checks.
```bash
./test_all.sh
```

### `test_cli.sh`
Original CLI test script (more complex version).
```bash
./test_cli.sh
```

### `fix_quality.sh`
Code quality fixer - formats code and reports linting issues.
```bash
./fix_quality.sh
```

## Usage

From the project root directory:
```bash
# Run comprehensive tests
./scripts/test_all.sh

# Run just CLI tests
./scripts/test_cli_simple.sh

# Fix code formatting
./scripts/fix_quality.sh
```

Or from the scripts directory:
```bash
cd scripts/
./test_all.sh           # Works from either location
./test_cli_simple.sh    # Works from either location
./fix_quality.sh        # Works from either location
```

All scripts automatically detect their location and adjust paths accordingly.

All scripts are executable and include colored output for better readability.