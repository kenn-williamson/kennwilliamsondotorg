# CLAUDE.md - Project Context

## Project Overview
This is a **Vue.js 3 + TypeScript + Vite** project located in the `vue-project/` folder. The project uses modern Vue 3 Composition API with TypeScript and Vite for fast development and building.

## Project Structure
```
vue-project/
├── src/
│   ├── components/          # Vue components (HeaderComponent, CounterBanner)
│   ├── assets/             # Static assets and CSS
│   ├── App.vue            # Main application component
│   └── main.ts            # Application entry point
├── public/                 # Public static files
├── package.json           # Dependencies and scripts
├── vite.config.ts         # Vite configuration
├── tsconfig.json          # TypeScript configuration
└── eslint.config.ts       # ESLint configuration
```

## Key Technologies
- **Vue 3.5.18** - Progressive JavaScript framework
- **TypeScript 5.8** - Type-safe JavaScript
- **Vite 7.0.6** - Fast build tool and dev server
- **ESLint + Prettier** - Code quality and formatting

## Available Scripts
- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run lint` - Run ESLint with auto-fix
- `npm run format` - Format code with Prettier

## Context7 MCP Server Usage
When working on this project, use the Context7 MCP server to look up APIs and documentation:

1. **For Vue.js APIs**: Use `resolve-library-id` with "vue" to get Vue.js documentation
2. **For TypeScript**: Use `resolve-library-id` with "typescript" for TypeScript language features
3. **For Vite**: Use `resolve-library-id` with "vite" for build tool configuration
4. **For ESLint/Prettier**: Use `resolve-library-id` with respective tool names

## Development Notes
- The app uses Vue 3 Composition API with `<script setup>` syntax
- Components are located in `src/components/` with a clean, modular structure
- TypeScript is configured for strict type checking
- Vite provides hot module replacement for fast development
- The project follows Vue.js best practices and conventions

## Current Application
The application is a "Time since last incident" counter landing page featuring:
- **HeaderComponent**: Welcome message for KennWilliamson.org
- **Smokey Image**: 512x768px image (smokey1.png) with responsive scaling
- **CounterBanner**: Real-time counter showing time elapsed since August 24, 2025
- Dynamic time display with years, months, weeks, days, hours, minutes, seconds
- Responsive design with flexbox layout and smart wrapping behavior
- Live updates every second using Vue reactivity
- Adaptive layout: stacks counter units vertically on screens ≤480px to prevent awkward wrapping

## Commit Convention
This project uses conventional commits with prefixes:
- **[FEATURE]**: New features and enhancements
- **[FIX]**: Bug fixes and corrections  
- **[CHORE]**: Maintenance, documentation, and tooling
- **[REFACTOR]**: Code restructuring without functional changes

## Future Considerations
This project may expand to include:
- Backend services in additional folders
- Alternative frontend implementations
- API integrations (use Context7 MCP server for API documentation)
- Database connections and data management
