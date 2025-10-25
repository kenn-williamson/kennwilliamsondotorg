/**
 * Hybrid API Architecture Diagram
 *
 * Shows the four distinct request pathways in the application:
 * 1. Built-in Session Route - nuxt-auth-utils automatic route (no backend call)
 * 2. Custom Auth Routes - Login/register/profile (hit Rust backend + manage session)
 * 3. SSR - Server-side rendering (uses session + fetches page data)
 * 4. Direct Backend - Client-side operations bypass Nuxt entirely
 */
export const architectureDiagram = `
graph TB
    subgraph clients["Client Layer"]
        Browser["🖥️ Browser*"]
    end

    subgraph nuxt["Nuxt Server Layer"]
        BuiltInSession["🔧 Built-in Session<br/>/api/_auth/session<br/>(nuxt-auth-utils)"]
        CustomAuth["🔧 Custom Auth Routes<br/>/api/auth/*"]
        SSR["🔧 SSR Engine<br/>(Page Rendering)"]
    end

    subgraph backend["Rust Backend"]
        RustAPI["⚡ Actix-web API<br/>(Stateless)"]
    end

    subgraph data["Data Layer"]
        PostgreSQL["🗄️ PostgreSQL<br/>(UUIDv7)"]
    end

    %% Single consolidated database connection
    RustAPI -->|"All data operations"| PostgreSQL

    %% Pathway 1: Built-in Session Route (Green - no backend)
    Browser -->|"① Get Session<br/>useUserSession()"| BuiltInSession
    BuiltInSession -->|"📋 Session from Cookie<br/>✅ No Backend/DB"| Browser

    %% Pathway 2: Custom Auth Routes (Steel - thick, hits backend)
    Browser ==>|"② Auth Operations<br/>login, register, profile"| CustomAuth
    CustomAuth ==>|"Forward Request"| RustAPI
    RustAPI ==>|"User Data + Tokens"| CustomAuth
    CustomAuth ==>|"🔒 Update Session Cookie<br/>✅ Persist Changes"| Browser

    %% Pathway 3: SSR Hydration (Blue - uses session + calls backend)
    Browser -->|"③ Initial Page Load"| SSR
    SSR -->|"Get Session"| BuiltInSession
    SSR -->|"Fetch Page Data"| RustAPI
    RustAPI -->|"JSON Data"| SSR
    SSR -->|"🌐 Server-Rendered HTML<br/>✅ SEO + Hydrated"| Browser

    %% Pathway 4: Direct Backend Calls (Cyan - dashed)
    Browser -.->|"④ Client Data Ops<br/>/backend/protected/*"| RustAPI
    RustAPI -.->|"⚡ JSON Response<br/>✅ Max Performance"| Browser

    classDef sessionStyle fill:#d1fae5,stroke:#059669,stroke-width:3px,color:#1e293b
    classDef customAuthStyle fill:#f1f5f9,stroke:#475569,stroke-width:3px,color:#1e293b
    classDef ssrStyle fill:#dbeafe,stroke:#2563eb,stroke-width:3px,color:#1e293b
    classDef apiStyle fill:#cffafe,stroke:#06b6d4,stroke-width:3px,color:#1e293b
    classDef dataStyle fill:#fef9c3,stroke:#f59e0b,stroke-width:2px,color:#1e293b

    class BuiltInSession sessionStyle
    class CustomAuth customAuthStyle
    class SSR ssrStyle
    class RustAPI apiStyle
    class PostgreSQL dataStyle
`
