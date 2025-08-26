# KennWilliamson.org Landing Page Implementation Plan

## Overview
Create a Vue 3 landing page with a header and dynamic "days without accident" style counter that updates in real-time.

## Requirements
- Header: "Welcome to KennWilliamson.org" (centered, full width)
- Banner: "Time since last incident" counter showing time elapsed since 8/24/25
- Counter displays: years, months, weeks, days, hours, minutes, seconds (only relevant units)
- Real-time updates every second
- Flexbox layout with responsive design
- Counter text wraps to new lines when needed

## Technical Approach

### 1. Component Architecture
- **HeaderComponent**: Simple centered header with welcome message
- **CounterBanner**: Dynamic counter component with time calculations
- **App.vue**: Main layout container using flexbox

### 2. Counter Logic
- Start date: August 24, 2025 (8/24/25)
- Calculate time difference using JavaScript Date objects
- Break down into units: years, months, weeks, days, hours, minutes, seconds
- Only display non-zero units for clean presentation
- Update every 1000ms using setInterval

### 3. CSS Flexbox Layout
```css
.app-container {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  align-items: center;
  justify-content: center;
}

.counter-banner {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 1rem;
}
```

### 4. Time Unit Display Logic
- Calculate total milliseconds difference
- Convert to years, months, weeks, days, hours, minutes, seconds
- Only show units with values > 0
- Format as readable strings (e.g., "1 week", "2 days")

### 5. Responsive Design
- Use flex-wrap for counter units to stack on smaller screens
- Center-align all content
- Appropriate spacing and typography

## Implementation Status
✅ **COMPLETED:**
1. Created HeaderComponent for welcome message
2. Created CounterBanner component with time calculation logic
3. Implemented real-time updates with Vue's lifecycle hooks
4. Styled components with flexbox CSS
5. Updated App.vue to use new components
6. Tested responsive behavior and time calculations
7. Updated banner title to "Time since last incident"

## Recent Changes
- **Text Update**: Changed banner title from "Days Without Incident" to "Time since last incident"
- **Documentation**: Updated CLAUDE.md with current application status and commit conventions
- **Image Integration**: Added smokey1.png (512x768) to landing page between header and counter
- **Responsive Layout**: Modified counter to use vertical stacking on screens ≤480px instead of wrapping
- **Smart Layout**: Prevents awkward text wrapping by switching to column layout on mobile devices

## Files to Modify/Create
- `src/components/HeaderComponent.vue` (new)
- `src/components/CounterBanner.vue` (new)
- `src/App.vue` (modify)
- `src/assets/main.css` (modify for global styles)