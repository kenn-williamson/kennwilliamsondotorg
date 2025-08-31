# Development Utilities Implementation

## Overview
Collection of focused development utilities for common tasks like password hashing, data generation, and maintenance operations. Each utility is isolated in its own directory with language-specific dependencies.

## Architecture Strategy

### Directory Structure
```
utils/
├── hash_gen/           # Rust bcrypt password hasher
│   ├── Cargo.toml
│   └── src/main.rs
├── [future utilities]
│   ├── slug_gen/       # Python slug generator
│   ├── db_seed/        # Node.js database seeding
│   └── image_resize/   # Image processing utility
```

### Design Principles
- **Single Responsibility**: Each utility focuses on one specific task
- **Language Flexibility**: Use the best tool for each job (Rust, Python, Node.js, etc.)
- **Isolated Dependencies**: No shared dependencies between utilities
- **Integration Ready**: Designed for use in development scripts and automation

## Implemented Utilities

### hash_gen - Bcrypt Password Hasher

**Purpose**: Generate bcrypt password hashes for development and testing

**Technology**: Rust + bcrypt crate  
**Location**: `utils/hash_gen/`

**Usage**:
```bash
cd utils/hash_gen
cargo run <password>
```

**Example**:
```bash
cargo run TestPassword1
# Output: $2b$04$GsxIUxkRm6rGJX15IeaY9ey5D5tcQatkob8.FPI6zasst1TD3zrXe
```

**Configuration**:
- **Cost**: 4 (optimized for development speed)
- **Algorithm**: bcrypt with random salt generation
- **Output**: Standard bcrypt hash format

**Integration Points**:
- `scripts/reset-db.sh`: Generates test user password hash
- Manual development tasks requiring bcrypt hashes
- Future user management scripts

**Technical Notes**:
- Cost (rounds) is embedded in the hash: `$2b$04$...` indicates cost 4
- Backend verification works regardless of generation cost
- Development uses cost 4 (fast), production uses cost 12 (secure)
- Hash format: `$2b$[cost]$[salt][hash]`

**Why Rust**: 
- Fast compilation and execution
- Same bcrypt implementation as backend
- Memory safety for security-sensitive operations
- Easy distribution as single binary

## Planned Utilities

### slug_gen - URL Slug Generator
**Technology**: Python  
**Purpose**: Generate URL-safe slugs from display names  
**Integration**: User registration, content management

### db_seed - Database Seeding
**Technology**: Node.js + PostgreSQL  
**Purpose**: Populate database with realistic test data  
**Integration**: Development environment setup

### image_resize - Image Processing
**Technology**: Python + Pillow or Rust + image crate  
**Purpose**: Resize and optimize user-uploaded images  
**Integration**: Future file upload features

## Development Workflow

### Adding New Utilities
1. Create dedicated subdirectory in `utils/`
2. Initialize with appropriate language tooling
3. Document in this file with usage examples
4. Add integration examples for scripts
5. Include in CI/CD if needed

### Integration Pattern
```bash
# Common pattern in development scripts
cd utils/[utility_name]
RESULT=$(command_to_run_utility 2>/dev/null)
if [ $? -ne 0 ]; then
    echo "❌ Utility failed"
    exit 1
fi
cd ../..
# Use $RESULT in script
```

### Maintenance
- Keep utilities focused and lightweight
- Update dependencies regularly for security
- Test utilities as part of development workflow
- Document breaking changes in this file

## Security Considerations

### Password Handling
- Never log or store passwords in plaintext
- Use appropriate cost factors for bcrypt
- Generate hashes in memory only
- Clear sensitive data after use

### Input Validation
- Validate all user inputs
- Sanitize file paths and commands
- Use parameterized queries for database operations
- Escape shell commands properly

---

**Status**: Active development - hash_gen implemented, additional utilities planned  
**Next**: Expand utility collection based on development needs