# Documentation Guidelines

## Purpose
This document establishes the standards and conventions for all documentation in the KennWilliamson.org project. Our documentation serves a dual purpose: providing clear guidance for human developers and acting as an efficient knowledge base for AI coding assistants. These guidelines ensure consistency, clarity, and optimal token usage while maintaining readability.

## Core Principles

### 1. Strategic Over Tactical
**Strategic** = WHY and WHAT (business goals, constraints, rationale)
**Tactical** = HOW (implementation details, current code structure)

- **CLAUDE.md**: Strategic context (business purpose, decisions, constraints)
- **ARCHITECTURE.md**: System-level decisions with rationale
- **IMPLEMENTATION-*.md**: Component-level decisions with rationale (Decision/Why/Alternatives/Trade-offs pattern)
- **Code**: Tactical implementation (discoverable)

**Documentation hierarchy**: Strategic context → Architectural decisions → Implementation decisions → Code

### 2. Durable Over Current
- Document decisions and rationale, not current code structure
- Strategic context survives refactoring (WHY doesn't change)
- Implementation details become stale quickly (HOW changes constantly)
- **Test**: Will this still be accurate after a major refactor? If no, make it more strategic.

### 3. Discoverable Not Documented
**Golden Rule**: Can Claude discover this by reading the code?
- If YES → Don't document it (Claude can read code faster and more completely)
- If NO → Document it (strategic context Claude needs)

Examples:
- ❌ Don't document: "We have a User model with email field" (discoverable)
- ✅ Do document: "We chose multi-table auth for GDPR compliance" (strategic rationale)

### 4. Dual Audience Design
- **Human Developers**: Assume familiarity with web/CS concepts but explain framework-specific details
- **AI Assistants**: Optimize for token efficiency while maintaining completeness
- **Balance**: Write concisely but include necessary context for understanding architectural decisions

### 5. Clear and Concise
- Focus on essential information without unnecessary verbosity
- Use simple language for concepts, technical language for specifics
- Each sentence should add value - remove filler words and redundant explanations

### 6. Aggressive Cross-Referencing
- **Never duplicate information** - use markdown cross-references instead
- Link format: `[Link Text](FILENAME.md#section-header)`
- Always use relative paths for internal documentation
- Create specific section anchors for precise linking

### 7. Separation of Concerns
- Each document should have a single, clear purpose
- Strategic context in CLAUDE.md
- System decisions in ARCHITECTURE.md
- Component decisions in IMPLEMENTATION-*.md files
- Future plans exclusively in ROADMAP.md

## Document Organization Strategy

### Core Documentation Structure
- **CLAUDE.md**: Strategic context for AI assistants (business purpose, decisions, constraints)
- **README.md**: Public-facing overview and getting started
- **ARCHITECTURE.md**: System-level design decisions and rationale
- **DEVELOPMENT-WORKFLOW.md**: Daily development processes, scripts, troubleshooting
- **CODING-RULES.md**: Development standards and conventions
- **DOCUMENTATION-GUIDELINES.md**: This document (meta-documentation standards)
- **UX-LAYOUT.md**: Design system and UI/UX guidelines
- **ROADMAP.md**: Future features and planned enhancements

### Implementation Documentation
**Purpose**: Document component-level decisions with rationale, not current code structure.

**Pattern**: Decision/Why/Alternatives/Trade-offs for each major choice.

- **IMPLEMENTATION-BACKEND.md**: Rust backend architecture decisions
- **IMPLEMENTATION-FRONTEND.md**: Nuxt.js frontend architecture decisions
- **IMPLEMENTATION-DATABASE.md**: PostgreSQL schema design decisions
- **IMPLEMENTATION-SECURITY.md**: Authentication and security decisions
- **IMPLEMENTATION-NGINX.md**: Reverse proxy configuration decisions
- **IMPLEMENTATION-DEPLOYMENT.md**: Production deployment decisions
- **IMPLEMENTATION-SCRIPTS.md**: Development automation script decisions
- **IMPLEMENTATION-TESTING.md**: Testing strategy and paradigm decisions
- **IMPLEMENTATION-LOGGING.md**: Logging and observability decisions
- **IMPLEMENTATION-UTILS.md**: Development utility decisions

### Feature Documentation Strategy
- **Brief Descriptions Only**: Feature implementations should be briefly described in their component docs
- **No Separate Feature Docs**: Avoid creating IMPLEMENTATION-FEATURE.md files
- **Cross-Component Features**: Document in the primary component, reference from others
- **Example**: Phrases system documented in IMPLEMENTATION-BACKEND.md, referenced from FRONTEND

## Decision Documentation Pattern

For all major architectural and implementation decisions, use this consistent pattern:

```markdown
### [Decision Name]
**Decision**: [What was chosen]

**Why**: [Rationale and constraints that drove this choice]

**Alternatives rejected**: [What else was considered and why it was rejected]

**Trade-offs**: [What you gain vs. what you lose with this choice]
```

**Example**:
```markdown
### UUIDv7 Primary Keys
**Decision**: Use pg_uuidv7 extension for all primary keys

**Why**:
- Time-ordered for better indexing than random UUIDs
- Globally unique (no conflicts in distributed systems)
- Future-proof for horizontal scaling

**Alternatives rejected**:
- Auto-increment integers: Risk of ID collisions in distributed systems
- UUID v4: Random ordering degrades index performance

**Trade-offs**: Slightly larger storage (128-bit vs 64-bit) vs. distributed-ready architecture
```

This pattern applies to:
- **CLAUDE.md**: High-level strategic decisions
- **ARCHITECTURE.md**: System-level architectural decisions
- **IMPLEMENTATION-*.md**: Component-level implementation decisions

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
- **Minimal code examples**: OK when clarifying confusing rules (see CODING-RULES.md)
- **Decision pattern examples**: OK in IMPLEMENTATION-* docs to illustrate the pattern being discussed
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
- **Purpose**: Document component-level decisions with rationale, not current code structure
- **Pattern**: Use Decision/Why/Alternatives/Trade-offs for each major choice
- **Structure**:
  - Overview (component purpose and strategic decisions)
  - Technology stack decisions (why this choice over alternatives)
  - Architecture decisions (why this pattern with rationale)
  - Key patterns (decisions that shape implementation)
  - Trade-offs (what you gain/lose with these choices)
- **Avoid**:
  - Current code structure (discoverable from code)
  - File trees and organization (Claude can explore)
  - Feature inventories (focus on decisions, not catalog)
  - Future plans (belongs in ROADMAP.md)

### Architecture Documents  
- **Purpose**: System design and deployment strategy
- **Focus**: How systems connect and interact
- **Include**: Resource allocation, service architecture, infrastructure

### Roadmap Document
- **Purpose**: Future features and planned changes
- **Structure**: Prioritized features with rationales
- **Include**: Architecture changes, new components, enhancement plans

## Content Organization Principles

### Strategic vs Tactical Content
- **CLAUDE.md**: Strategic WHY/WHAT (business goals, constraints, decisions)
- **ARCHITECTURE.md**: System-level decisions with rationale
- **IMPLEMENTATION-*.md**: Component-level decisions with rationale
- **Code**: Tactical HOW (implementation)
- **Roadmap**: Future decisions and plans

### File References vs Code Examples
- **Preferred**: `See authentication handlers in backend/src/routes/auth.rs`
- **Minimal examples OK**: When clarifying confusing rules (CODING-RULES.md)
- **Decision examples OK**: When illustrating pattern discussed (IMPLEMENTATION-*.md)
- **Avoid**: Comprehensive code tutorials (let Claude read the actual code)

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
- Strategic context for AI assistants (WHY and WHAT, not HOW)
- Business purpose, user needs, architectural decisions, constraints
- Answers: "Why does this project exist? What makes it unique?"
- Aggressive cross-referencing to detailed decision documentation
- **Not**: Feature inventory, current state descriptions, how-to guides

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
- Component-level decisions with rationale (Decision/Why/Alternatives/Trade-offs pattern)
- Strategic "why" for this component, not current "how" (code is discoverable)
- Focus on decisions that shape implementation
- Cross-references for related decisions in other components

---

*This document serves as the authoritative guide for all documentation standards in the project. When in doubt, refer to these guidelines and update them if new patterns emerge.*