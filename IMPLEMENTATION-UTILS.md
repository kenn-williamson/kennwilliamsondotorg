# Development Utilities Implementation

## Overview
Utility architecture philosophy and design decisions.

## Architecture Philosophy

### Single-Purpose Utilities
**Decision**: One utility = one task

**Why:**
- Clear responsibility
- Easy to understand
- Simple to maintain
- No shared dependencies

**Alternative rejected:**
- Monolithic utility library: Becomes unwieldy
- Shared dependencies: Coupling issues

### Language Flexibility
**Decision**: Use best language per utility

**Why:**
- Rust for performance (hash generation)
- Python for scripting (if needed)
- JavaScript for frontend tools (if needed)
- Right tool for the job

**Trade-offs:**
- Multiple toolchains
- Worth it: Each utility optimized

### Isolated Dependencies
**Decision**: Each utility has own dependencies

**Why:**
- No dependency conflicts
- Update independently
- Delete utility = delete dependencies
- Clear ownership

**Pattern:**
- Each utility in own directory
- Own Cargo.toml, package.json, requirements.txt
- Self-contained

## Implemented Utilities

### hash_gen - Password Hasher
**Purpose**: Generate bcrypt hashes for development

**Why Rust:**
- Same library as backend (consistent)
- Fast compilation
- Type safety

**Why needed:**
- Test data requires hashed passwords
- Manual hashing error-prone
- Scripts need automation

**Design decision:**
- Cost factor 4 for development (fast)
- Backend uses 12 for production (secure)
- Balance: Development speed vs production security

## Integration Strategy

### Script Integration
**Decision**: Utilities called from development scripts

**Why:**
- Automation over manual use
- Consistent workflows
- Documented usage
- Error handling in scripts

**Pattern:**
- Script calls utility
- Script handles output
- Script provides user feedback

### No Complex CLI
**Decision**: Simple argument passing, no fancy CLI

**Why:**
- Used by scripts, not humans directly
- Simplicity over features
- Less code to maintain

**Trade-offs:**
- Not user-friendly for direct use
- Worth it: Scripts provide friendly interface

## Security Considerations

### Sensitive Data Handling
**Decision**: Never log sensitive data

**Why:**
- Prevent accidental exposure
- Development logs can leak
- Security by default

**Pattern:**
- Accept password as argument
- Output hash only
- No intermediate logging

### Cleanup
**Decision**: Utilities don't persist sensitive data

**Why:**
- No files written
- No cache created
- Ephemeral by design

## Maintenance Philosophy

### Add When Needed
**Decision**: Create utilities reactively, not proactively

**Why:**
- YAGNI principle
- Solve actual problems
- Don't build unused tools

**When to add:**
- Same task repeated 3+ times
- Script logic getting complex
- Reusable across projects

### Delete Freely
**Decision**: Remove unused utilities

**Why:**
- No obligation to maintain
- Clutter is technical debt
- Easy to recreate if needed

**Pattern:**
- If not used in 6 months, consider deletion
- Git history preserves if needed later
