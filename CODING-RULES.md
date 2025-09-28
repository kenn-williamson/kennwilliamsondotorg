# Coding Rules and Standards

## Overview
This document establishes coding standards and development conventions for the KennWilliamson.org project to ensure consistency, quality, and maintainability across all languages and frameworks.

## General Coding Principles

### Code Quality
- Use idiomatic code for the appropriate language or framework
- Code should prefer clarity over cleverness
- Follow language-specific conventions and best practices
- Implement comprehensive error handling
- Write code that is self-documenting through clear naming

### Problem-Solving Approach
- When repeatedly encountering errors, use Context7 MCP to look up documentation to ensure correct implementation according to package intentions
- When encountering technical challenges, use web search tools to research solutions before suggesting alternatives or compromises
- Many specific technical issues have existing solutions that can be found through targeted research

### Decision Making
- Ask questions instead of assuming when core decisions are involved
- Challenge potentially problematic requests with reasoned objections
- Balance thoroughness with practical progress
- Focus on current implementation - avoid referencing "old behavior" or "previous approaches" when making changes

## Language-Specific Standards

### TypeScript/JavaScript
- **Mode**: Use TypeScript strict mode
- **Typing**: Implement proper typing throughout the application
- **Vue.js**: Use Composition API with `<script setup>` syntax
- **Error Handling**: Implement proper error boundaries and validation

### Rust
- **Conventions**: Follow standard Rust conventions and idioms
- **Error Handling**: Implement comprehensive error handling with proper Result types
- **Async**: Use async/await patterns consistently
- **Testing**: Add integration tests for new API endpoints

### SQL/Database
- **Migrations**: Use SQLx migrations for all schema changes
- **Queries**: Use parameterized queries to prevent SQL injection
- **Naming**: Use snake_case for database identifiers

## Framework-Specific Standards

### Nuxt.js/Vue.js
- **Components**: Use Composition API consistently
- **State Management**: Implement proper Pinia store patterns
- **Forms**: Use VeeValidate + Yup for validation
- **Routing**: Follow file-based routing conventions

### Actix-web/Rust Backend
- **Middleware**: Use idiomatic middleware patterns
- **Routes**: Implement clear route organization
- **Authentication**: Follow JWT best practices
- **Database**: Use SQLx with compile-time query checking

## Development Workflow Standards

### Version Control
- **Commit Messages**: Follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/#specification) specification
  - `feat:` - New features and enhancements
  - `fix:` - Bug fixes and corrections
  - `docs:` - Documentation changes
  - `style:` - Code style changes (formatting, missing semicolons, etc.)
  - `refactor:` - Code restructuring without functional changes
  - `perf:` - Performance improvements
  - `test:` - Adding or updating tests
  - `build:` - Build system or dependency changes
  - `ci:` - CI/CD configuration changes
  - `chore:` - Maintenance tasks and tooling
  - Use `!` suffix or `BREAKING CHANGE:` footer for breaking changes
  - Optional scope in parentheses: `feat(auth):`, `fix(database):`

### Development Environment
- **Scripts**: Always use development scripts instead of manual Docker commands
- **Environment**: Use `.env.development` for all development work
- **Health Checks**: Check service health after major changes
- **Migrations**: Run migrations safely with `setup-db.sh` (preserves data)
- **Cache Management**: Update SQLx cache after SQL query changes

### Error Handling Strategy
- **Bash Commands**: When a bash command fails with "No such file or directory", first check `pwd` to verify current location
- **Script Usage**: Update development scripts rather than falling back to direct CLI commands

## Security Standards

### Authentication and Authorization
- **Passwords**: Use bcrypt with appropriate cost factors
- **JWT**: Implement secure token handling and validation
- **Input Validation**: Sanitize and validate all user inputs
- **Database**: Use parameterized queries exclusively

### Data Protection
- **Secrets**: Never commit secrets or environment variables to version control
- **Logging**: Avoid logging sensitive information
- **CORS**: Configure CORS appropriately for environment

## Testing Standards

### Backend Testing
- **Integration Tests**: Comprehensive test coverage for all API endpoints
- **Database**: Use test database with proper isolation
- **Cleanup**: Implement proper test cleanup and data management

### Frontend Testing
- **Unit Tests**: Component-level testing (planned)
- **Integration Tests**: User flow testing (planned)
- **E2E Testing**: Full application workflow testing (planned)

## Documentation Standards

### Code Documentation
- **Comments**: Code should be self-documenting; use comments sparingly for complex logic
- **README Files**: Keep implementation-specific details in separate documentation
- **File References**: Reference actual code files rather than duplicating code in documentation
- **Separation of concerns**: Maintain separation of concerns between documents to avoid repetition and confusion

### API Documentation
- **Contracts**: Maintain clear API contracts with JSON schema representations
- **Examples**: Provide realistic request/response examples
- **Versioning**: Document API changes and maintain backward compatibility

## Performance and Optimization

### Frontend Optimization
- **Bundle Size**: Monitor and optimize JavaScript bundle sizes
- **Loading**: Implement proper loading states and error handling
- **Caching**: Use appropriate caching strategies for API calls

### Backend Optimization
- **Database**: Optimize queries and use proper indexing
- **Connection Pooling**: Implement efficient database connection management
- **Memory**: Monitor memory usage and implement proper resource cleanup

### Development Environment
- **Hot Reload**: Maintain efficient development environment with minimal rebuild times
- **Resource Usage**: Monitor Docker container resource consumption

## Quality Assurance

### Code Review Standards
- **Clarity**: Code should be readable and maintainable
- **Testing**: New features should include appropriate tests
- **Documentation**: Update relevant documentation with code changes
- **Standards Compliance**: Ensure adherence to project coding standards

### Continuous Improvement
- **Refactoring**: Regularly refactor code to improve maintainability
- **Dependencies**: Keep dependencies updated and secure
- **Performance**: Monitor and address performance issues proactively

---

*These coding standards serve as guidelines for maintaining code quality and consistency throughout the project. They should be reviewed and updated as the project evolves.*