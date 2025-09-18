# Development Utilities Implementation

## Overview
Focused utilities for development tasks. Each utility is self-contained with its own dependencies.

## Architecture Strategy

### Directory Structure
```
utils/
├── hash_gen/           # Rust bcrypt password hasher
│   ├── Cargo.toml
│   └── src/main.rs
└── [additional utilities as needed]
```

### Design Principles
- **Single Responsibility**: Each utility focuses on one specific task
- **Language Flexibility**: Use the best tool for each job (Rust, Python, Node.js, etc.)
- **Isolated Dependencies**: No shared dependencies between utilities
- **Integration Ready**: Designed for use in development scripts and automation

## Implemented Utilities

### hash_gen - Bcrypt Password Hasher

**Purpose**: Generate bcrypt hashes for development
**Technology**: Rust + bcrypt
**Location**: `utils/hash_gen/`

**Usage**:
```bash
cd utils/hash_gen
cargo run <password>
# Output: $2b$04$...
```

**Details**:
- Cost factor 4 for development speed
- Compatible with backend's bcrypt verification
- Used by `scripts/reset-db.sh` for test data

## Development Integration

### Usage in Scripts
Utilities are designed to be called from development scripts and automation workflows.

### Adding New Utilities
1. Create subdirectory in `utils/`
2. Add language-specific setup
3. Document in this file
4. Test integration

## Security Notes
- Never log plaintext passwords
- Clear sensitive data after use
- Validate all inputs
- See [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md) for security guidelines