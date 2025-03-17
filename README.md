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