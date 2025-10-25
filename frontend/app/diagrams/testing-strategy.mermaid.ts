/**
 * Testing Strategy Diagram
 *
 * Shows the comprehensive testing pyramid across all layers (~600 tests total):
 * - Frontend Layer: Component, composable, store, and service tests
 * - API Layer: End-to-end HTTP tests with actix-test
 * - Service Layer: Business logic tests with mock repositories (fast!)
 * - Repository Layer: Integration tests with real PostgreSQL
 *
 * Paradigm-based approach: Each layer tested at the appropriate level
 */
export const testingStrategyDiagram = `
graph TB
    FrontendTests["<b>Frontend Layer (~175 tests)</b><br/>Component Tests<br/>Action Composable Tests<br/>Pure Store Tests<br/>Service Tests"]
    APITests["<b>Backend API Layer (~150 tests)</b><br/>End-to-End HTTP Tests<br/>Route Handler Tests<br/>Middleware Tests<br/>(actix-test framework)"]
    ServiceTests["<b>Service Layer (~200 tests)</b><br/>Business Logic Tests<br/>Mock Repositories<br/>Fast Execution<br/>(mockall framework)"]
    RepoTests["<b>Repository Layer (~100 tests)</b><br/>Integration Tests<br/>Real PostgreSQL<br/>SQL Query Tests<br/>(testcontainers)"]

    FrontendTests -->|"Calls API"| APITests
    APITests -->|"Uses services"| ServiceTests
    ServiceTests -->|"Mocks repository"| RepoTests

    APITests -.->|"Integration: Real DB"| RepoTests

    classDef frontendStyle fill:#dbeafe,stroke:#2563eb,stroke-width:3px,color:#1e293b,font-size:13px
    classDef apiStyle fill:#cffafe,stroke:#06b6d4,stroke-width:3px,color:#1e293b,font-size:13px
    classDef serviceStyle fill:#d1fae5,stroke:#059669,stroke-width:3px,color:#1e293b,font-size:13px
    classDef repoStyle fill:#fef9c3,stroke:#f59e0b,stroke-width:3px,color:#1e293b,font-size:13px

    class FrontendTests frontendStyle
    class APITests apiStyle
    class ServiceTests serviceStyle
    class RepoTests repoStyle
`
