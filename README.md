# ULID Tool

A command-line utility for working with ULIDs and BINARY(16) columns in MySQL and other databases.

## Background

I created this tool to solve a practical problem in my personal projects. I needed an easy way to look up records stored with ULID identifiers in BINARY(16) database columns. After struggling with manual conversions between string ULIDs and their binary representation, I built this utility to streamline my database lookup workflow.

## What is ULID?

[ULID](https://github.com/ulid/spec) (Universally Unique Lexicographically Sortable Identifier) is a compact, time-ordered alternative to UUID. ULIDs are especially useful for database primary keys and can be stored efficiently in BINARY(16) columns.

## Features

- Convert between ULID string format and hexadecimal format
- Generate SQL queries to find records by ULID in any table
- Copy SQL directly to clipboard (macOS support)
- Work with any table and column that uses BINARY(16) to store ULIDs

## Installation

### Prerequisites

- Rust and Cargo
- Git

### Steps

```bash
# Clone the repository
git clone https://github.com/thanipro/ulid-db-tool.git
cd ulid-db-tool

# Install the tool
./install.sh
```

## Usage

```bash
# Display help and available commands
ulid

# Convert between ULID and hex formats
ulid convert 01J5KERCT1VJCKV1VEVZZ3NFY4

# Generate SQL to find a record by ULID
ulid find 01J5KERCT1VJCKV1VEVZZ3NFY4 --table users

# Generate SQL and copy to clipboard
ulid db 01J5KERCT1VJCKV1VEVZZ3NFY4 --table products --column product_id --copy
```

### Command Reference

#### Convert Command

```bash
ulid convert <ULID_OR_HEX>
```

Detects whether the input is a ULID string or hexadecimal value and converts between the formats.

#### Find Command

```bash
ulid find <ULID> --table <TABLE_NAME> [--column <COLUMN_NAME>]
```

Generates SQL to find a record by ULID in the specified table.

#### DB Command

```bash
ulid db <ULID> --table <TABLE_NAME> [--column <COLUMN_NAME>] [--copy]
```

Generates a SQL query and optionally copies it to the clipboard.

## Building from Source

If you prefer to build the tool manually:

```bash
# Clone the repository
git clone https://github.com/thanipro/ulid-db-tool.git
cd ulid-db-tool

# Build with cargo
cargo build --release

# The executable will be in target/release/ulid
```