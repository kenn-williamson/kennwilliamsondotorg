/**
 * Intelligence Augmentation Workflow Diagram
 *
 * Shows the human-AI collaboration cycle used throughout this project:
 * - Human provides requirements and validates output
 * - Claude generates code and explores patterns
 * - Tests provide fast feedback
 * - Iteration happens continuously until feature is complete
 * - Successful features deploy to production
 */
export const iaWorkflowDiagram = `
graph TB
    Start["👤 Human: Define Requirements & Architecture"] --> Generate
    Generate["🤖 Claude: Generate Code & Explore Patterns"] --> Validate
    Validate["👤 Human: Validate Output Against Specs"] --> Tests
    Tests["🧪 Fast Feedback: Run Tests (Unit/Integration)"] --> Results
    Results["📊 Analyze: Review Results & Identify Gaps"]

    Results -->|"🔄 Iterate: Continue learning"| Generate
    Results -->|"⚠️ Gaps Found: Course Correct"| CourseCorrect
    Results -->|"✅ Feature Complete & Tested"| Deploy
    CourseCorrect["👤 Human: Refine Requirements"] --> Generate
    Deploy["🚀 Deploy to Production<br/>Real users, real value"]

    classDef humanStyle fill:#dbeafe,stroke:#2563eb,stroke-width:3px,color:#1e293b,font-size:14px
    classDef claudeStyle fill:#d1fae5,stroke:#059669,stroke-width:3px,color:#1e293b,font-size:14px
    classDef feedbackStyle fill:#fef9c3,stroke:#f59e0b,stroke-width:3px,color:#1e293b,font-size:14px
    classDef outcomeStyle fill:#dcfce7,stroke:#16a34a,stroke-width:4px,color:#1e293b,font-size:14px

    class Start,Validate,CourseCorrect humanStyle
    class Generate claudeStyle
    class Tests,Results feedbackStyle
    class Deploy outcomeStyle
`
