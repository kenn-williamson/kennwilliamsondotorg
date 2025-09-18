# Documentation Guidelines

## Purpose
This document establishes the standards and conventions for all documentation in the KennWilliamson.org project. Our documentation serves a dual purpose: providing clear guidance for human developers and acting as an efficient knowledge base for AI coding assistants. These guidelines ensure consistency, clarity, and optimal token usage while maintaining readability.

## Core Principles

### 1. Dual Audience Design
- **Human Developers**: Assume familiarity with web/CS concepts but explain framework-specific details
- **AI Assistants**: Optimize for token efficiency while maintaining completeness
- **Balance**: Write concisely but include necessary context for understanding architectural decisions

### 2. Clear and Concise
- Focus on essential information without unnecessary verbosity
- Use simple language for concepts, technical language for specifics
- Each sentence should add value - remove filler words and redundant explanations

### 3. Aggressive Cross-Referencing
- **Never duplicate information** - use markdown cross-references instead
- Link format: `[Link Text](FILENAME.md#section-header)`
- Always use relative paths for internal documentation
- Create specific section anchors for precise linking

### 4. Separation of Concerns
- Each document should have a single, clear purpose
- Implementation details in IMPLEMENTATION-*.md files
- Architecture decisions in ARCHITECTURE.md
- Future plans exclusively in ROADMAP.md

### 5. Current State Documentation
- Implementation documents describe what IS, not what WILL BE
- Remove all future tense and planning language from implementation docs
- No TODO items or planned features in implementation documents

### 6. No Code Duplication
- **NEVER include code snippets in documentation**
- Reference actual files: `backend/src/routes/auth.rs`
- Reference sections: `See JWT validation in backend/src/middleware/auth.rs`
- Exception: JSON schema representations for API contracts (language-agnostic)

## Document Organization Strategy

### Core Documentation Structure
- **README.md**: Public-facing overview and getting started
- **ARCHITECTURE.md**: System design, service architecture, infrastructure decisions
- **DEVELOPMENT-WORKFLOW.md**: Daily development processes, scripts, troubleshooting
- **CODING-RULES.md**: Development standards and conventions
- **DOCUMENTATION-GUIDELINES.md**: This document
- **UX-LAYOUT.md**: Design system and UI/UX guidelines
- **ROADMAP.md**: Future features and planned enhancements
- **PROJECT_HISTORY.md**: Completed phases and lessons learned

### Implementation Documentation
- **IMPLEMENTATION-BACKEND.md**: Rust/Actix-web API implementation
- **IMPLEMENTATION-FRONTEND.md**: Nuxt.js/Vue frontend implementation
- **IMPLEMENTATION-DATABASE.md**: PostgreSQL schema and management
- **IMPLEMENTATION-SECURITY.md**: Authentication, authorization, and security measures
- **IMPLEMENTATION-NGINX.md**: Reverse proxy and SSL configuration
- **IMPLEMENTATION-DEPLOYMENT.md**: Production deployment process
- **IMPLEMENTATION-SCRIPTS.md**: Development and deployment scripts
- **IMPLEMENTATION-DATA-CONTRACTS.md**: API contracts and JSON schemas
- **IMPLEMENTATION-TESTING.md**: Test architecture and coverage
- **IMPLEMENTATION-UTILS.md**: Development utilities and tools

### Feature Documentation Strategy
- **Brief Descriptions Only**: Feature implementations should be briefly described in their component docs
- **No Separate Feature Docs**: Avoid creating IMPLEMENTATION-FEATURE.md files
- **Cross-Component Features**: Document in the primary component, reference from others
- **Example**: Phrases system documented in IMPLEMENTATION-BACKEND.md, referenced from FRONTEND

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

## Writing for Efficiency

### Token Optimization Techniques
- **Remove Filler**: "It's important to note that" → Just state the fact
- **Active Voice**: "The backend is implemented by Rust" → "Rust implements the backend"
- **Combine Related Points**: Use semicolons and commas instead of multiple sentences
- **Avoid Redundancy**: Don't repeat information available via cross-reference

### Framework-Specific Documentation
- **Assume Basic Knowledge**: Don't explain what React/Vue/Rust is
- **Focus on Implementation**: How we use the framework, not what it does
- **Highlight Unusual Patterns**: Document deviations from standard practices
- **Version-Specific Features**: Note when using framework-specific features

### Examples of Efficient Writing
```
❌ Poor: "The authentication system is implemented using JWT tokens. JWT tokens are used because they provide stateless authentication. The JWT tokens expire after 1 hour for security reasons."

✅ Better: "JWT authentication with 1-hour expiration provides stateless security."

❌ Poor: "To run the development environment, you need to execute the dev-start.sh script which is located in the scripts directory."

✅ Better: "Run development: `./scripts/dev-start.sh`"
```

## Special Document Rules

### CLAUDE.md
- Lightweight entry point for AI assistants
- Project context and high-level overview only
- Aggressive cross-referencing to detailed documentation

### README.md
- Public-facing for GitHub visitors
- Getting started guide with clear steps
- Links to detailed documentation

### ARCHITECTURE.md
- System-wide design decisions
- Service interaction patterns
- Infrastructure and deployment architecture
- Resource allocation strategies

### ROADMAP.md
- Future features with rationales
- Priority tracking
- No implementation details

### IMPLEMENTATION-*.md
- Current state only
- Technical details for specific components
- Cross-references for shared concepts

---

*This document serves as the authoritative guide for all documentation standards in the project. When in doubt, refer to these guidelines and update them if new patterns emerge.*