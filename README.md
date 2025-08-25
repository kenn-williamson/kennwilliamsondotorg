# KennWilliamson.org

A Vue.js 3 landing page featuring a real-time "Time since last incident" counter.

## Overview

This project is a modern web application built with Vue 3, TypeScript, and Vite that displays a dynamic counter showing the time elapsed since August 24, 2025. The counter updates in real-time and displays years, months, weeks, days, hours, minutes, and seconds in a responsive layout.

## Features

- **Real-time Counter**: Updates every second with precise time calculations
- **Responsive Design**: Flexbox layout that adapts to different screen sizes
- **Modern Stack**: Vue 3 Composition API with TypeScript and Vite
- **Clean Architecture**: Modular component structure
- **Live Updates**: No page refresh required

## Technology Stack

- **Vue 3.5.18** - Progressive JavaScript framework
- **TypeScript 5.8** - Type-safe JavaScript
- **Vite 7.0.6** - Fast build tool and dev server
- **ESLint + Prettier** - Code quality and formatting

## Local Development

### Prerequisites

- Node.js v20.19.4 or higher
- npm (comes with Node.js)

### Setup Instructions

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd kennwilliamsondotorg
   ```

2. **Navigate to the Vue project**
   ```bash
   cd vue-project
   ```

3. **Install dependencies**
   ```bash
   npm install
   ```

4. **Start the development server**
   ```bash
   npm run dev
   ```

5. **Open your browser**
   - The application will be available at `http://localhost:5173`
   - The dev server supports hot module replacement for instant updates

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run lint` - Run ESLint with auto-fix
- `npm run format` - Format code with Prettier

## Project Structure

```
vue-project/
├── src/
│   ├── components/          # Vue components
│   │   ├── HeaderComponent.vue
│   │   └── CounterBanner.vue
│   ├── assets/             # Static assets and CSS
│   ├── App.vue            # Main application component
│   └── main.ts            # Application entry point
├── public/                 # Public static files
└── package.json           # Dependencies and scripts
```

## Contributing

This project uses conventional commits with the following prefixes:
- `[FEATURE]` - New features and enhancements
- `[FIX]` - Bug fixes and corrections
- `[CHORE]` - Maintenance, documentation, and tooling
- `[REFACTOR]` - Code restructuring without functional changes

## License

This project is for personal use.