# UX & Layout Design Document - KennWilliamson.org

## Design Philosophy

**Core Principles:** Truth, Beauty, Love
**Aesthetic Approach:** Understated elegance with layered influences
**Key Characteristic:** Nothing loud or in-your-face; subtle sophistication

## Visual Aesthetic Framework

### Primary Influences by Page/Section
- **Homepage:** Sacred/Gothic with construction motifs
- **Authentication Pages:** Clean, minimal with subtle sacred elements
- **Incidents Feature:** Technology aesthetic with geometric patterns
- **About Me:** Frontier/Nature with Japanese traditional influences
- **Future Features:** Rotate through aesthetic themes as appropriate

### Implemented Theme: Nautical Steampunk
- **Color Psychology:** Blue/cyan for trust and professionalism
- **Metallic Elements:** Silver/steel industrial accents
- **Reserved Warm Tones:** Gold accents for premium/special features only
- **Warm Steampunk:** Reserved for Incidents feature (brass/copper/mahogany aesthetic)

### Color Palette & Semantic Design Tokens

#### Semantic Color System
The site uses a semantic color token system defined in `frontend/app/assets/css/main.css` using Tailwind's `@theme` directive. This enables theme-wide color changes from a single source of truth.

#### Color Families

**Primary (Blue Spectrum - Trust & Professionalism)**
- `primary-50` through `primary-900` (Tailwind blue family)
- Usage: Buttons, CTAs, links, headings, strong text emphasis
- Key shades: `primary-600` (#2563eb), `primary-700` (#1d4ed8)

**Accent (Cyan - Nautical Highlights)**
- `accent-50` through `accent-700` (Tailwind cyan family)
- Usage: Interactive highlights, hover states, accent elements
- Key shades: `accent-300` (#67e8f9), `accent-400` (#22d3ee), `accent-500` (#06b6d4)

**Nautical (Slate/Steel - Industrial Aesthetic)**
- `nautical-50` through `nautical-900` (Tailwind slate family)
- Usage: Dark backgrounds, borders, industrial styling
- Key shades: `nautical-900` (#0f172a), `nautical-800` (#1e293b), `nautical-600` (#475569)

**Gold (Premium Accents - Sparingly Used)**
- `gold-50` through `gold-600` (Tailwind yellow/amber family)
- Usage: Premium features, special emphasis only
- Key shades: `gold-400` (#fbbf24), `gold-500` (#f59e0b)

#### Semantic Aliases (Recommended Usage)
```css
/* Text Colors */
text-text-primary      /* Primary narrative text */
text-text-secondary    /* Supporting text */
text-text-muted        /* Disabled/muted text */
text-text-link         /* Interactive links */

/* Backgrounds */
bg-surface-base        /* Primary white/light background */
bg-surface-elevated    /* Card/panel backgrounds */

/* Borders */
border-border-light    /* Subtle borders */
border-border-accent   /* Accent emphasis borders */

/* Interactive States */
interactive-default    /* Button normal state */
interactive-hover      /* Button hover state */
interactive-active     /* Button active state */
```

#### Implementation Notes
- All core components use semantic tokens exclusively
- Theme changes require editing only `app/assets/css/main.css`
- No component files need modification for theme updates
- See `COLOR-MIGRATION-GUIDE.md` for migration patterns

### Design Token Philosophy

**Single Source of Truth:**
- All colors defined in `frontend/app/assets/css/main.css`
- Components reference semantic tokens, not raw colors
- Theme-wide changes without touching component files

**Three-Tier Token Hierarchy:**
1. **Semantic Aliases** (Preferred): `text-text-link`, `bg-surface-elevated`
2. **Named Color Families**: `primary-600`, `accent-300`, `nautical-900`
3. **Raw Tailwind Colors** (Avoid): `blue-600`, `cyan-300`, `slate-900`

**Benefits:**
- Maintainability: Single file to update for theme changes
- Consistency: Uniform naming across codebase
- Scalability: Easy to add variants or switch themes
- Future-proof: Following Tailwind CSS 4+ best practices

### Typography Hierarchy
- **Headers:** Ornate, decorative treatment reflecting section aesthetic
- **Body Text:** Clean, readable serif or serif-inspired fonts
- **UI Elements:** Modern, clean sans-serif for functionality

### Visual Elements
- **Stained Glass:** Subtle window-like accents, not dominant
- **Geometric Patterns:** Metallic textures in tech sections
- **Iconography:** Orthodox/medieval inspired but simplified
- **Architecture:** Gothic arches, Romanesque curves as subtle frame elements

## Responsive Design Strategy

### Breakpoints (Content-First Approach)
```css
/* Mobile Portrait */
@media (max-width: 479px)

/* Mobile Landscape / Small Tablet */
@media (min-width: 480px) and (max-width: 767px)

/* Tablet */
@media (min-width: 768px) and (max-width: 1023px)

/* Laptop/Desktop */
@media (min-width: 1024px) and (max-width: 1439px)

/* Large Desktop */
@media (min-width: 1440px)
```

### Design Approach
- **Equally Important:** Mobile and desktop UX
- **Screen Agnostic:** Elements adapt fluidly across sizes
- **Content-Driven:** Breakpoints based on content needs, not devices

## Navigation Architecture

### Header Layout (Sticky)
```
[Logo/Site Name]     [Nav: About | Incidents]     [Auth: Avatar ↓ | Sign In | Register]
```

### Navigation States

#### Unauthenticated
- **Left:** "KennWilliamson" (clickable logo/title)
- **Center:** Horizontal nav bar: "About" | "Incidents"
- **Right:** "Sign In" | "Register" buttons

#### Authenticated
- **Left:** "KennWilliamson" (clickable logo/title)
- **Center:** Horizontal nav bar: "About" | "Incidents" 
- **Right:** Avatar circle (first letter of display name) with dropdown
  - Account Settings
  - Sign Out
  - (Future: Profile, Preferences, etc.)

#### Mobile Responsive
- **Hamburger Menu:** Collapses nav on mobile screens
- **Maintained Hierarchy:** Logo left, hamburger right
- **Dropdown Integration:** Avatar dropdown works within mobile menu

### Navigation Flexibility
- **Page Independence:** Features don't rely on specific nav structure
- **Adaptable:** Can transition between horizontal, sidebar, or mobile without breaking pages
- **Scalable:** Easy to add new feature pages

## Page Layouts

### Homepage
- **Layout:** Centered content approach
- **Hero Section:** Under construction message with castle construction image
- **Aesthetic:** Sacred/Gothic with construction motifs
- **Image Source:** Royalty-free castle under construction (Unsplash/Pixabay)
- **Future:** Space for branding elements, call-to-actions

### Authentication Pages (Login/Register)
- **Layout:** Centered form with subtle background
- **Aesthetic:** Clean, minimal with subtle sacred elements
- **Form Design:** Traditional yet modern input styling
- **Validation:** Inline validation with gentle error states

### Profile Management Page
- **Layout:** Two-form architecture with clean sections
- **Aesthetic:** Clean, minimal following authentication page styling
- **Form Design:** Account information and security forms with real-time validation
- **Navigation:** Accessible via avatar dropdown → "Profile Settings"

### Incidents Feature Page
- **Layout:** Dashboard-style interface for CRUD operations
- **Aesthetic:** Technology theme with geometric patterns and metallic textures
- **Primary Feature:** Currently the main feature page
- **Components:** Timer display, management interface, creation forms

### Public Incident Timer Display (`/incident-timer/[user_slug]`)
- **Layout:** Clean, focused timer display
- **Aesthetic:** Minimal tech with subtle geometric elements
- **Purpose:** Public-facing timer view (no authentication required)
- **Design:** Large, readable timer with minimal distractions

### About Me Page
- **Layout:** Long-form content with sections
- **Aesthetic:** Frontier/Nature with Japanese traditional influences
- **Future Content:** Personal information, portfolio elements
- **Design Elements:** Subtle nature motifs, traditional patterns

## Component Design System

### Header Component
- **Sticky Behavior:** Remains at top during scroll
- **Responsive:** Collapses to hamburger on mobile
- **Avatar:** Circular with letter, dropdown on click
- **States:** Handles authenticated/unauthenticated seamlessly

### Navigation Component
- **Horizontal Bar:** Primary navigation method
- **Active States:** Clear indication of current page
- **Extensible:** Easy to add new feature pages
- **Responsive:** Integrates with mobile hamburger menu

### Form Components
- **Styling:** Consistent with overall aesthetic
- **Validation:** Real-time validation with clear error states
- **Accessibility:** Full keyboard navigation, screen reader support
- **Security:** Proper input sanitization, CSRF protection

### Timer Components
- **Display:** Large, readable timer format
- **Controls:** Intuitive start/stop/reset interface
- **Management:** CRUD interface for authenticated users
- **Public View:** Clean, distraction-free public display

### Accordion & Tooltip Components
- **Metallic Plaque Styling:** 3D inset shadow effects for depth
- **Variants:** Steel (default), Naval Brass (cyan), Bronze (blue), Gold (premium)
- **Visual Effect:** Gradient backgrounds with highlight/shadow layers
- **Usage:** Primarily in About pages for expandable content and contextual information
- **Accessibility:** Full keyboard support, ARIA attributes, screen reader friendly
- **Base Components:** BaseAccordion.vue and BaseTooltip.vue provide unstyled functionality
- **Styled Components:** SteampunkAccordion.vue and SteampunkTooltip.vue add nautical theme

### Footer Component
- **Nautical Theme:** Matches header with dark slate gradients
- **Geometric Pattern:** Repeating grid overlay (20px intervals)
- **Metallic Border:** Gradient top border effect
- **Content:** Copyright, GitHub badge, legal links
- **Responsive:** Stacks vertically on mobile
- **GitHub Badge:** Links to open-source repository with hover effects

## User Experience Flows

### Registration Flow
1. Click "Register" → Registration page
2. Fill form with validation feedback
3. Submit → Backend processing
4. Success → Redirect to homepage (authenticated state)
5. Header updates to show avatar

### Login Flow
1. Click "Sign In" → Login page
2. Enter credentials with validation
3. Submit → Authentication
4. Success → Redirect to homepage (authenticated state)
5. Header updates to show avatar with dropdown

### Incident Management Flow
1. Navigate to "Incidents" → Dashboard view
2. Create new timer → Form interface
3. Manage existing timers → CRUD operations
4. Public sharing → Generate shareable URL
5. View public timer → Clean display page

### Account Management Flow
1. Click avatar → Dropdown appears
2. "Profile Settings" → Profile management page
3. Edit display name/slug → Real-time validation and preview
4. Change password → Current password verification required
5. "Sign Out" → Logout process
6. Return to homepage → Unauthenticated state

## Technical Considerations

### Performance
- **Image Optimization:** Responsive images with proper sizing
- **Code Splitting:** Route-based code splitting for faster loads
- **Caching:** Appropriate cache headers for static assets
- **Bundle Size:** Monitor and optimize bundle sizes

### Accessibility
- **WCAG 2.1 AA:** Target compliance level
- **Keyboard Navigation:** Full keyboard accessibility
- **Screen Readers:** Proper ARIA labels and structure
- **Color Contrast:** Ensure sufficient contrast ratios

### SEO
- **Meta Tags:** Appropriate meta descriptions and titles
- **Open Graph:** Social media sharing optimization
- **Structured Data:** JSON-LD for rich snippets
- **Site Map:** XML sitemap for search engines

## Future Enhancements

### Navigation Evolution
- **Sidebar Option:** Potential sidebar navigation for more features
- **Breadcrumbs:** For deeper page hierarchies
- **Search:** Site-wide search functionality
- **Favorites:** User-customizable quick access

### Visual Enhancements
- **Animations:** Subtle, tasteful transitions
- **Theming:** User-selectable themes while maintaining brand
- **Interactive Elements:** Hover states, micro-interactions
- **Progressive Enhancement:** Advanced features for capable browsers

### Content Areas
- **Blog/Articles:** Long-form content area
- **Portfolio:** Project showcase area
- **Contact:** Contact form and information
- **Admin Panel:** Administrative interface (admin users)

---

*This document serves as the design foundation for the KennWilliamson.org frontend implementation. It should be referenced alongside IMPLEMENTATION-FRONTEND.md for technical implementation details.*