# Phrase Feature UX Working Document

## Overview
UX design specifications for the integrated incident timer and phrase management system. This document outlines the complete user experience for the 5-tab interface that replaces the current `/incidents` page.

## Tab System Architecture

### 5-Tab Mechanical Switching Interface
```
┌─────────────────────────────────────────────────────────┐
│  ⚙️ Incident Timer Command Station                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────┐│
│  │ ⏱️ TIMER│ │ ⚙️ CTRL │ │ 📝 PHRASE│ │ ⚙️ FILTER│ │ 📋 ││
│  │    [●]  │ │    [○]  │ │    [○]  │ │    [○]  │ │[○] ││
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────┘│
│         │         │         │         │         │      │
│         ▼         ▼         ▼         ▼         ▼      │
│  ┌─────────────────────────────────────────────────┐   │
│  │                                                 │   │
│  │           TAB CONTENT AREA                      │   │
│  │                                                 │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

## Tab 1: Timer Display (Public Page Style)

### Design Concept: "Steampunk Timer Showcase"
**Aesthetic**: Public page aesthetic with steampunk banner and timer display

### Layout:
```
┌─────────────────────────────────────────────────────────┐
│  ⏱️ Timer Display Station                               │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  [SteampunkBanner Component]                    │   │
│  │  "Vigilance Maintained - Until Next Challenge"  │   │
│  │  (Static until manual refresh)                  │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  🕐 Current Timer Status                        │   │
│  │  ┌─────────────────────────────────────────────┐ │   │
│  │  │  [Steampunk Clock Display]                  │ │   │
│  │  │  "2 days, 14 hours, 23 minutes"            │ │   │
│  │  │  Since: System maintenance incident        │ │   │
│  │  └─────────────────────────────────────────────┘ │   │
│  │  [Refresh Phrase] [Share Timer]                  │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### Features:
- **SteampunkBanner Component**: Displays current phrase (static until refresh)
- **Timer Display**: Current steampunk clock component
- **Manual Refresh**: Button to get new random phrase
- **Share Functionality**: Copy public timer URL
- **Public Page Aesthetic**: Matches the public timer page styling

## Tab 2: Timer Controls (Current Incidents Functionality)

### Design Concept: "Steam-Powered Control Panel"
**Aesthetic**: Industrial control station with current incidents page functionality

### Layout:
```
┌─────────────────────────────────────────────────────────┐
│  ⚙️ Timer Control Station                               │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  🔧 Quick Actions                               │   │
│  │  [Reset Timer] [Add Note] [Create New Timer]    │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  📊 Current Timer Management                    │   │
│  │  ┌─────────────────────────────────────────────┐ │   │
│  │  │  • System maintenance - 3 days ago         │ │   │
│  │  │    Notes: "Database migration completed"   │ │   │
│  │  │    [Edit] [Delete] [Reset]                 │ │   │
│  │  └─────────────────────────────────────────────┘ │   │
│  │  ┌─────────────────────────────────────────────┐ │   │
│  │  │  • Security incident - 1 week ago          │ │   │
│  │  │    Notes: "Unauthorized access detected"   │ │   │
│  │  │    [Edit] [Delete] [Reset]                 │ │   │
│  │  └─────────────────────────────────────────────┘ │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### Features:
- **Reset Timer**: Reset current timer with optional notes
- **Edit Timer**: Modify existing timer details
- **Delete Timer**: Remove timer from history
- **Create New Timer**: Start a new timer
- **Timer History**: List of all timers with actions
- **Notes Management**: Add/edit notes for timers

## Tab 3: Phrase Suggestions (Writing Desk)

### Design Concept: "Victorian Writing Desk"
**Aesthetic**: Victorian writing desk with silver/gold accents and gear elements

### Layout:
```
┌─────────────────────────────────────────────────────────┐
│  📝 Timer Phrase Suggestion Box                        │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  "Inscribe your words of wisdom for incident timers"    │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  🖋️ Your Phrase Suggestion                      │   │
│  │  ┌─────────────────────────────────────────────┐ │   │
│  │  │                                             │ │   │
│  │  │  [Large text area with vintage scroll      │ │   │
│  │  │   styling, silver border, gold highlights] │ │   │
│  │  │                                             │ │   │
│  │  └─────────────────────────────────────────────┘ │   │
│  │  Words: 0/100  Characters: 0/500                 │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  [Seal with Wax] [Discard Draft]                        │
│                                                         │
│  📜 Your Previous Submissions:                          │
│  ┌─────────────────────────────────────────────────┐   │
│  │ "Vigilance Maintained - Until Next Challenge"   │   │
│  │ Status: ⏳ Pending Review  •  Submitted 2d ago  │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### Features:
- **Writing Desk Interface**: Victorian styling with silver/gold accents
- **Character Counter**: Real-time word and character counting
- **Draft Saving**: Auto-save as user types
- **Validation**: Check for duplicates, length, content
- **Submission History**: List of previous suggestions with status
- **Gear Placeholders**: Ready for SVG/PNG gear elements

## Tab 4: Phrase Filter (Control Panel)

### Design Concept: "Industrial Filter Station"
**Aesthetic**: Industrial control panel with mechanical toggle switches

### Layout:
```
┌─────────────────────────────────────────────────────────┐
│  ⚙️ Phrase Filter Station                               │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  "Configure which phrases appear in your timer display" │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  🔍 Search phrases... [________________]        │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  Phrase: "Vigilance Maintained - Until Next..." │   │
│  │  ┌─────────────────────────────────────────────┐ │   │
│  │  │  ⚙️ [●]──────────────[○] ⚙️                │ │   │
│  │  │     Show in Timer   Hide                   │ │   │
│  │  └─────────────────────────────────────────────┘ │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  Phrase: "The Higher the Ascent, the Sheerer..."│   │
│  │  ┌─────────────────────────────────────────────┐ │   │
│  │  │  ⚙️ [○]──────────────[●] ⚙️                │ │   │
│  │  │     Show in Timer   Hide                   │ │   │
│  │  └─────────────────────────────────────────────┘ │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  [Save All Changes] [Reset to Default] [Select All]    │
└─────────────────────────────────────────────────────────┘
```

### Features:
- **Mechanical Toggle Switches**: Silver/gold toggles with gear elements
- **Search Functionality**: Find specific phrases quickly
- **Bulk Actions**: Select all, reset to default
- **Real-time Updates**: Changes reflect immediately
- **Visual Feedback**: Clear show/hide indicators
- **Gear Placeholders**: Ready for SVG/PNG gear elements

## Tab 5: Suggestion History (Filing Cabinet)

### Design Concept: "Status Tracking System"
**Aesthetic**: Filing cabinet with status indicators and admin feedback

### Layout:
```
┌─────────────────────────────────────────────────────────┐
│  📋 Suggestion History & Status                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  Filter: [All ▼] Search: [________] [🔍]       │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  ✅ "Vigilance Maintained - Until Next..."      │   │
│  │  Status: Approved  •  Added to timer phrases   │   │
│  │  Submitted: 3 days ago  •  Admin: system       │   │
│  │  ┌─────────────────────────────────────────────┐ │   │
│  │  │  "Great addition to our wisdom collection!" │ │   │
│  │  └─────────────────────────────────────────────┘ │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  ❌ "Keep Your Head Down and Work Hard"         │   │
│  │  Status: Rejected  •  Not added to collection  │   │
│  │  Submitted: 1 week ago  •  Admin: system       │   │
│  │  ┌─────────────────────────────────────────────┐ │   │
│  │  │  "Too similar to existing phrase"           │ │   │
│  │  └─────────────────────────────────────────────┘ │   │
│  │  [Edit & Resubmit] [Delete]                      │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  ⏳ "The Path Forward is Through Adversity"     │   │
│  │  Status: Pending Review  •  Under consideration │   │
│  │  Submitted: 1 day ago                           │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### Features:
- **Status Tracking**: Pending, Approved, Rejected indicators
- **Admin Feedback**: Comments from admin reviewers
- **Edit & Resubmit**: Modify rejected suggestions
- **Filter & Search**: Find specific submissions
- **Timeline**: Chronological submission history
- **Bulk Actions**: Delete multiple submissions

## Mobile Responsive Design

### Mobile Tab Layout:
```
┌─────────────────┐
│ ⚙️ Timer Mgmt   │
├─────────────────┤
│ ┌─┐ ┌─┐ ┌─┐ ┌─┐ ┌─┐│
│ │⏱️│ │⚙️│ │📝│ │⚙️│ │││
│ │[●]│ │[○]│ │[○]│ │[○]│ │[○]││
│ └─┘ └─┘ └─┘ └─┘ └─┘│
│                 │
│   TAB CONTENT   │
│                 │
└─────────────────┘
```

### Mobile Features:
- **Horizontal Scroll**: Tab bar scrolls horizontally if needed
- **Swipe Gestures**: Swipe left/right to switch tabs
- **Touch-Friendly**: Large touch targets for toggles
- **Responsive Forms**: Forms adapt to mobile screen sizes
- **Collapsible Sections**: Long content sections can be collapsed

## Navigation & State Management

### URL Structure:
- `/incidents` → Shows Tab 1 (Timer Display) by default
- `/incidents?tab=controls` → Shows Tab 2 (Timer Controls)
- `/incidents?tab=suggestions` → Shows Tab 3 (Phrase Suggestions)
- `/incidents?tab=filter` → Shows Tab 4 (Phrase Filter)
- `/incidents?tab=history` → Shows Tab 5 (Suggestion History)

### State Persistence:
- **Active Tab**: Persists on page refresh
- **Form Data**: Individual forms maintain data when switching tabs
- **Search Filters**: Maintained across tab switches
- **User Preferences**: Saved to user profile

### Navigation Flow:
- **Default**: Tab 1 (Timer Display) on first visit
- **Tab Switching**: Mechanical animation between tabs
- **Form Validation**: Prevents tab switching with invalid data
- **Auto-Save**: Forms auto-save when switching tabs

## Integration Points

### Public Timer Page:
- **Banner Integration**: Uses same SteampunkBanner component
- **Phrase Randomization**: Respects user's exclusion preferences
- **Consistent Styling**: Matches private page aesthetic

### Admin Features:
- **Separate Admin Page**: `/admin/phrases` for admin management
- **Suggestion Review**: Admin can approve/reject suggestions
- **Phrase Management**: Admin can add/edit/remove phrases
- **User Management**: Admin can manage user roles and permissions

## Visual Design System

### Color Palette:
- **Primary**:  Prussian blue  #003153.
- **Accent Metals**: Gold (#FFD700 family) and Silver (#C0C0C0 family)
- **Supporting**: Rich, muted earth tones and deep blues
- **Text**: High contrast for readability

### Typography:
- **Headers**: Ornate, decorative treatment reflecting steampunk aesthetic
- **Body Text**: Clean, readable serif or serif-inspired fonts
- **UI Elements**: Modern, clean sans-serif for functionality

### Mechanical Elements:
- **Toggle Switches**: Silver base with gold accents, gear icons
- **Tab Switching**: Mechanical animation with gear transitions
- **Gear Placeholders**: Ready for SVG/PNG gear elements
- **Steam Effects**: Subtle steam animations on hover

## Accessibility Features

### Keyboard Navigation:
- **Tab Order**: Logical tab order through all interactive elements
- **Focus Management**: Clear focus indicators and logical flow
- **Form Navigation**: Easy keyboard navigation through forms

### Screen Reader Support:
- **ARIA Labels**: Proper labels for all interactive elements
- **Status Announcements**: Clear status updates for form submissions
- **Descriptive Text**: Clear descriptions for all UI elements

### Visual Accessibility:
- **High Contrast**: Clear visual indicators for all states
- **Color Independence**: Information not conveyed by color alone
- **Scalable Text**: Text scales appropriately with browser zoom

## Implementation Phases

### Phase 1: Core Tab System
- [ ] Create 5-tab mechanical switching interface
- [ ] Implement Tab 1 (Timer Display) with SteampunkBanner
- [ ] Implement Tab 2 (Timer Controls) with current functionality
- [ ] Basic mobile responsive design

### Phase 2: Phrase Management
- [ ] Implement Tab 3 (Phrase Suggestions) with writing desk
- [ ] Implement Tab 4 (Phrase Filter) with mechanical toggles
- [ ] Implement Tab 5 (Suggestion History) with status tracking
- [ ] Form validation and state management

### Phase 3: Polish & Integration
- [ ] Add gear SVG/PNG elements
- [ ] Implement swipe gestures for mobile
- [ ] Add animations and transitions
- [ ] Admin page integration
- [ ] Accessibility improvements

## Success Metrics

### User Experience:
- [ ] Intuitive tab switching with mechanical feel
- [ ] Seamless form data persistence across tabs
- [ ] Clear visual feedback for all actions
- [ ] Mobile-friendly touch interactions

### Functionality:
- [ ] Complete phrase suggestion workflow
- [ ] Effective phrase filtering system
- [ ] Comprehensive suggestion history tracking
- [ ] Integration with existing timer functionality

### Technical:
- [ ] Responsive design across all breakpoints
- [ ] Accessible interface for all users
- [ ] Fast tab switching and form interactions
- [ ] Clean, maintainable code structure

---

*This document serves as the comprehensive UX specification for the phrase feature integration. It should be referenced during implementation and updated as design decisions are made.*
