# Documentation Guidelines

## Purpose
This document establishes the standards and conventions for all documentation in the KennWilliamson.org project to ensure consistency, clarity, and maintainability.

## Core Principles

### 1. Clear and Concise
- Documentation should be clear and concise
- Focus on essential information without unnecessary verbosity
- Use simple language and avoid jargon when possible

### 2. Separation of Concerns
- Cleanly separate concerns as much as possible
- Cross-reference documentation sections rather than repeat information
- Avoid duplication between documents

### 3. Current State Focus
- Implementation documents should focus on current implementation only
- Future plans and features belong in the roadmap document
- Avoid mixing planning with current state documentation

### 4. No Code Duplication
- **NEVER include code snippets in documentation**
- Reference actual files instead of replicating code
- Code examples should use file references with section names
- Exception: JSON schema representations for API contracts (language-agnostic)

## File Naming Conventions

### Document Categories
- **CLAUDE.md**: Main project context for Claude Code tool (lightweight entry point)
- **ARCHITECTURE.md**: System design and deployment strategy
- **ROADMAP.md**: Future features and planned architecture
- **UX-LAYOUT.md**: Design system and responsive guidelines
- **PROJECT_HISTORY.md**: Completed phases and lessons learned
- **README.md**: Public-facing project overview
- **IMPLEMENTATION-\*.md**: Technical implementation details per component
- **DOCUMENTATION-GUIDELINES.md**: This document (documentation standards)
- **CODING-RULES.md**: Development standards and conventions
- **DEVELOPMENT-WORKFLOW.md**: Daily development processes and script usage

### Naming Standards
- Use UPPERCASE for main category documents
- Use hyphens to separate words
- Implementation docs follow pattern: `IMPLEMENTATION-[COMPONENT].md`
- Be descriptive but concise in file names

## Markdown Formatting Standards

### Headers
- Use descriptive headers that can serve as anchor links
- Follow logical hierarchy (H1 → H2 → H3)
- Use sentence case for headers
- Avoid special characters in headers (they become anchor links)

### Lists
- Use `-` for unordered lists
- Use `1.` for ordered lists
- Be consistent within each document
- Use sub-lists sparingly for clarity

### Code References
- Reference files using: `backend/src/routes/auth.rs`
- Reference file sections using: `backend/src/routes/auth.rs (Authentication handlers section)`
- Never include actual code snippets
- Use backticks for file names and technical terms

### Status Indicators
- **🚧 Under Construction**: For features currently being implemented
- **Remove all ✅ checkmarks**: Implementation docs should not have completion status
- **Use present tense**: Describe current state, not past achievements

## Cross-Reference Standards

### Internal Links
- Use standard markdown linking: `[link text](FILENAME.md#section-name)`
- Section anchors use lowercase with hyphens: `#authentication-system`
- Always use relative file paths
- Test links to ensure they work

### Cross-Reference Examples
```markdown
See [Authentication System](IMPLEMENTATION-BACKEND.md#authentication-system)
For deployment details, refer to [ARCHITECTURE.md](ARCHITECTURE.md#deployment-strategy)
```

### Avoid @ Symbol
- **Never use `@FILENAME.md`** - this is Claude Code tool syntax
- Use full markdown links instead

## Document Structure Guidelines

### Implementation Documents
- **Purpose**: Document current technical implementation only
- **Structure**: 
  - Overview (what is currently implemented)
  - Technology stack (actual versions in use)
  - Current features (working functionality)
  - File structure (actual directory layout)
  - Integration points (how it connects to other components)
- **Avoid**: Future plans, TODO items, planning sections

### Architecture Documents  
- **Purpose**: System design and deployment strategy
- **Focus**: How systems connect and interact
- **Include**: Resource allocation, service architecture, infrastructure

### Roadmap Document
- **Purpose**: Future features and planned changes
- **Structure**: Prioritized features with rationales
- **Include**: Architecture changes, new components, enhancement plans

## Content Organization Principles

### Current vs Future State
- **Implementation docs**: Current state only
- **Roadmap**: Future state only  
- **Architecture**: Current architecture with cross-references to roadmap for changes

### File References vs Code Snippets
- **Preferred**: `See authentication handlers in backend/src/routes/auth.rs`
- **Avoid**: Copying actual Rust/TypeScript/SQL code into documentation
- **Exception**: JSON request/response schemas (language-agnostic contracts)

### Cross-Document Relationships
- Use markdown links to connect related information
- Prefer deep linking to specific sections
- Create logical navigation paths between documents

## Maintenance Guidelines

### Regular Updates
- Update documentation when implementation changes
- Verify cross-references remain valid after file moves
- Keep roadmap updated as features are completed

### Review Process
- Check for code snippets that should be file references
- Verify current state accuracy in implementation docs
- Ensure cross-references work and point to correct sections

### Quality Checks
- No duplicate information across documents
- Clear separation between current and future state
- All links functional and pointing to correct locations
- Consistent formatting and naming conventions

## Special Document Rules

### CLAUDE.md
- Keep lightweight - main purpose is Claude Code tool entry point
- Use cross-references to other documents instead of duplicating content
- Focus on project context and tool-specific information

### README.md
- Public-facing document for GitHub visitors
- High-level overview without implementation details
- Getting started guide for new developers
- Link to relevant documentation for deeper information

### ROADMAP.md
- Living document that changes as priorities shift
- Include rationale for feature priorities
- Cross-reference current implementation when relevant
- Track features in progress vs planned

---

*This document serves as the authoritative guide for all documentation standards in the project. When in doubt, refer to these guidelines and update them if new patterns emerge.*