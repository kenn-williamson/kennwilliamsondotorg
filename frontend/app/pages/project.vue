<template>
  <div class="min-h-screen nautical-background">
    <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">

      <!-- Hero Section -->
      <section class="mb-12">
        <div class="text-center mb-8">
          <h1 class="text-4xl sm:text-5xl lg:text-6xl font-bold text-primary-100 leading-tight mb-4">
            Building KennWilliamson.org
          </h1>
          <p class="text-xl sm:text-2xl text-accent-300 font-semibold mb-4">
            Intelligence Augmentation in Practice
          </p>
          <p class="text-lg text-nautical-200 max-w-3xl mx-auto">
            Two months part-time, working with Claude (Anthropic) to learn Rust and Nuxt simultaneously. This proves that IA enables rapid learning when paired with strong fundamentals and active validation.
          </p>
        </div>

        <!-- Key Metrics -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 max-w-4xl mx-auto">
          <div class="metric-card">
            <div class="text-3xl font-bold text-accent-300 mb-1">2 Months</div>
            <div class="text-sm text-nautical-300">Part-Time Dev</div>
          </div>
          <div class="metric-card">
            <div class="text-3xl font-bold text-accent-300 mb-1">600+ Tests</div>
            <div class="text-sm text-nautical-300">All Layers</div>
          </div>
          <div class="metric-card">
            <div class="text-3xl font-bold text-accent-300 mb-1">Zero→Prod</div>
            <div class="text-sm text-nautical-300">Rust/Nuxt</div>
          </div>
          <div class="metric-card">
            <div class="text-3xl font-bold text-accent-300 mb-1">Live AWS</div>
            <div class="text-sm text-nautical-300">Production</div>
          </div>
        </div>
      </section>

      <!-- Main Layout with Sidebar -->
      <div class="flex flex-col lg:flex-row gap-8">
        <!-- Sidebar Navigation -->
        <aside class="lg:w-64 flex-shrink-0">
          <div class="lg:sticky lg:top-20">
            <!-- Mobile: Dropdown Menu -->
            <div class="lg:hidden mb-4">
              <button
                @click="mobileMenuOpen = !mobileMenuOpen"
                class="w-full bg-gradient-to-r from-nautical-700 to-primary-800 border-2 border-nautical-500 px-4 py-3 rounded-lg flex items-center justify-between text-primary-100 font-semibold"
              >
                <span>{{ currentSection }}</span>
                <span :class="{ 'rotate-180': mobileMenuOpen }" class="transition-transform">▼</span>
              </button>

              <Transition name="menu-slide">
                <nav v-if="mobileMenuOpen" class="mt-2 bg-gradient-to-br from-nautical-700 to-primary-800 border-2 border-nautical-500 rounded-lg p-4">
                  <ul class="space-y-2">
                    <li v-for="section in sections" :key="section.id">
                      <a
                        :href="`#${section.id}`"
                        :class="[
                          'block px-3 py-2 rounded transition-colors',
                          activeSection === section.id
                            ? 'bg-gradient-to-r from-primary-600 to-accent-600 text-white font-semibold'
                            : 'text-nautical-200 hover:bg-nautical-600/50 hover:text-accent-300'
                        ]"
                        @click="mobileMenuOpen = false"
                      >
                        {{ section.title }}
                      </a>
                    </li>
                  </ul>
                </nav>
              </Transition>
            </div>

            <!-- Desktop: Sidebar -->
            <nav class="hidden lg:block bg-gradient-to-br from-nautical-700 to-primary-800 border-2 border-nautical-500 rounded-lg p-4 shadow-lg">
              <h2 class="text-lg font-bold text-primary-100 mb-4 pb-2 border-b-2 border-accent-400">
                Technical Overview
              </h2>
              <ul class="space-y-1">
                <li v-for="section in sections" :key="section.id">
                  <a
                    :href="`#${section.id}`"
                    :class="[
                      'block px-3 py-2 rounded transition-all duration-200',
                      activeSection === section.id
                        ? 'bg-gradient-to-r from-primary-600 to-accent-600 text-white font-semibold shadow-md shadow-accent-500/50'
                        : 'text-nautical-200 hover:bg-nautical-600/50 hover:text-accent-300 hover:translate-x-1'
                    ]"
                  >
                    {{ section.title }}
                  </a>
                </li>
              </ul>
            </nav>
          </div>
        </aside>

        <!-- Main Content Card -->
        <main class="flex-1 min-w-0">
          <article class="content-card">
            <!-- Decorative Header Border -->
            <div class="h-2 bg-gradient-to-r from-primary-600 via-indigo-600 to-primary-700"></div>

            <div class="p-6 sm:p-8 lg:p-12">
              <div class="prose-custom">

            <!-- Section 1: Project Overview -->
            <section id="overview" class="mb-12 scroll-mt-24">
              <h2 class="section-heading">Project Overview</h2>

              <p class="text-nautical-800 leading-relaxed mb-4">
                KennWilliamson.org serves multiple strategic goals as both a portfolio demonstration and
                personal platform. This full-stack application showcases professional deployment skills
                while serving as a learning environment for AI-assisted development in an unfamiliar technology stack.
              </p>

              <div class="feature-grid">
                <div class="feature-item">
                  <h3 class="feature-title">Portfolio Demonstration</h3>
                  <p class="feature-text">Production-ready full-stack application demonstrating professional engineering practices, deployment expertise, and architectural decision-making.</p>
                </div>
                <div class="feature-item">
                  <h3 class="feature-title">Learning Platform</h3>
                  <p class="feature-text">Exploring AI-assisted development in Rust and Nuxt.js (unfamiliar technologies) while applying strong architectural knowledge from years of enterprise experience.</p>
                </div>
                <div class="feature-item">
                  <h3 class="feature-title">Live Production</h3>
                  <p class="feature-text">Deployed at <a href="https://kennwilliamson.org" class="text-primary-600 hover:text-primary-800 underline">kennwilliamson.org</a> on AWS t3.small instance with SSL, demonstrating real-world deployment capabilities.</p>
                </div>
                <div class="feature-item">
                  <h3 class="feature-title">Personal Platform</h3>
                  <p class="feature-text">Public testimony platform with community features (incident timers, motivational phrases) and steampunk aesthetic reflecting personal creative vision.</p>
                </div>
              </div>
            </section>

            <!-- Section 2: The IA Laboratory -->
            <section id="ia-laboratory" class="mb-12 scroll-mt-24">
              <h2 class="section-heading">The IA Laboratory</h2>

              <p class="text-nautical-800 leading-relaxed mb-4">
                This site isn't just a portfolio. It's a proof-of-concept for the Intelligence Augmentation framework I evangelize professionally. In my talks to manufacturing, oil & gas, and healthcare companies, I explain that GMs (generative models) amplify skilled humans rather than replace them. This project demonstrates that thesis in action.
              </p>

              <div class="border-l-4 border-primary-500 pl-4 bg-primary-50 p-4 rounded-r-lg mb-6">
                <p class="text-sm text-nautical-700 mb-2">
                  <strong class="text-primary-800">The Challenge:</strong> Learn Rust and Nuxt.js simultaneously (two technologies I'd never used professionally) and build a production full-stack application with proper architecture, comprehensive testing, and real deployment infrastructure.
                </p>
                <p class="text-sm text-nautical-700">
                  <strong class="text-primary-800">The Timeline:</strong> Two months, part-time (evenings and weekends while juggling work, single dad responsibilities, and three kids' schedules).
                </p>
              </div>

              <p class="text-nautical-800 leading-relaxed mb-4">
                Working with Claude (Anthropic) throughout this project, I applied the IA methodology I recommend to clients: human judgment drives decisions, GMs accelerate execution, active validation catches mistakes. The result? A learning curve that would have taken 6+ months in the pre-GM era compressed into 8 weeks part-time.
              </p>

              <div class="feature-grid">
                <div class="feature-item">
                  <h3 class="feature-title">What IA Enabled</h3>
                  <p class="feature-text">Rapid prototyping, pattern exploration, boilerplate generation, and learning acceleration. Claude helped me explore Rust idioms, Nuxt SSR patterns, and PostgreSQL optimization techniques I'd never encountered.</p>
                </div>
                <div class="feature-item">
                  <h3 class="feature-title">What Required Human Judgment</h3>
                  <p class="feature-text">Every architectural decision (3-layer pattern, hybrid API design, normalized auth schema). Every security choice (PKCE OAuth, JWT strategy). Every course correction when Claude's suggestions didn't fit requirements.</p>
                </div>
                <div class="feature-item">
                  <h3 class="feature-title">The IA Pattern in Practice</h3>
                  <p class="feature-text">Claude generates → I validate against requirements → Test reveals gaps → Iterate. For complex features, I used TDD: write tests first, Claude implements, I validate behavior matches specification.</p>
                </div>
                <div class="feature-item">
                  <h3 class="feature-title">Learning, Not Autopilot</h3>
                  <p class="feature-text">This wasn't Claude writing code while I watched. I learned Rust ownership, lifetimes, and traits. I understand Nuxt SSR hydration, server-side routing, and composition API patterns. IA accelerated learning, it didn't replace it.</p>
                </div>
              </div>

              <p class="text-sm text-nautical-600 italic mt-4">
                <strong>For the technical audience:</strong> This page documents the full journey: what worked, what didn't, course corrections we made, and lessons learned about practicing IA in a complex learning environment. If you're evaluating IA for your team or exploring GM-assisted development, the details below show what the process actually looks like.
              </p>

              <!-- IA Workflow Diagram -->
              <div class="mt-8">
                <h4 class="text-lg font-bold text-primary-800 mb-4 text-center">The IA Collaboration Cycle</h4>

                <!-- Desktop: Show diagram -->
                <div class="hidden md:block">
                  <MermaidDiagram :diagram="iaWorkflowDiagram" />
                </div>

                <!-- Mobile: Show message -->
                <div class="block md:hidden bg-gradient-to-br from-nautical-50 to-primary-50 border-2 border-nautical-300 rounded-lg p-6 text-center">
                  <p class="text-sm text-nautical-700 mb-2">
                    📱 Workflow diagram available on larger screens
                  </p>
                  <p class="text-xs text-nautical-600">
                    View this page on a tablet or desktop to see the IA collaboration cycle diagram
                  </p>
                </div>

                <p class="text-xs text-nautical-600 italic text-center mt-4">
                  <strong>The Human-AI Feedback Loop:</strong> Human judgment drives decisions and validates output. Claude accelerates execution and explores patterns. Tests provide fast feedback. Iteration happens continuously. This cycle ran hundreds of times over 2 months.
                </p>
              </div>
            </section>

            <!-- Section 3: What the Site Does -->
            <section id="what-it-does" class="mb-12 scroll-mt-24">
              <h2 class="section-heading">What the Site Does</h2>

              <p class="text-nautical-800 leading-relaxed mb-6">
                This isn't just a technical demonstration. Real users interact with real features requiring real architecture decisions.
              </p>

              <div class="space-y-6">
                <div class="border-l-4 border-primary-500 pl-4 bg-primary-50 p-4 rounded-r-lg">
                  <h3 class="text-lg font-bold text-primary-800 mb-2">Incident Timers</h3>
                  <p class="text-sm text-nautical-700 mb-2">
                    Users track recovery milestones (days since last incident). Each user controls their timer's visibility through two settings: "Show timer at my personal slug?" and "Show timer in the public directory?" Simple feature on the surface.
                  </p>
                  <p class="text-sm text-nautical-700">
                    <strong class="text-primary-800">What it drove:</strong> Feature-based role system (<code class="bg-nautical-100 px-1 text-xs">email_verified</code> to create/edit timers, preventing spam accounts), user preference architecture, public/protected API routing patterns.
                  </p>
                </div>

                <div class="border-l-4 border-accent-500 pl-4 bg-accent-50/30 p-4 rounded-r-lg">
                  <h3 class="text-lg font-bold text-primary-800 mb-2">Motivational Phrases</h3>
                  <p class="text-sm text-nautical-700 mb-2">
                    Community members suggest inspirational quotes. Admin reviews and approves via moderation panel. Approved phrases appear in daily rotation for all users. Straightforward workflow.
                  </p>
                  <p class="text-sm text-nautical-700">
                    <strong class="text-primary-800">What it drove:</strong> Domain event system with observer pattern, AWS SES integration for approval notifications, email template architecture (Askama compile-time verification), admin panel with moderation queue.
                  </p>
                </div>

                <div class="border-l-4 border-primary-600 pl-4 bg-sky-50 p-4 rounded-r-lg">
                  <h3 class="text-lg font-bold text-primary-800 mb-2">Personal Testimony & Access Requests</h3>
                  <p class="text-sm text-nautical-700 mb-2">
                    Public platform for professional work and faith journey. Personal content (Origins, The Wilderness, Finding Faith, etc.) requires <code class="bg-nautical-100 px-1 text-xs">trusted-contact</code> role. Users request access, I approve manually.
                  </p>
                  <p class="text-sm text-nautical-700">
                    <strong class="text-primary-800">What it drove:</strong> SEO requirements (Nuxt SSR for search indexing), role-based content gating, access request workflow with email notifications, data privacy architecture (GDPR/CCPA self-service deletion and export).
                  </p>
                </div>
              </div>

              <p class="text-sm text-nautical-600 italic mt-6">
                These straightforward user features required sophisticated technical architecture. The sections below explain how we built them and what we learned along the way.
              </p>
            </section>

            <!-- Section 4: Architecture -->
            <section id="architecture" class="mb-12 scroll-mt-24">
              <h2 class="section-heading">Architecture</h2>

              <div class="mb-6">
                <h3 class="subsection-heading">3-Layer Backend Architecture</h3>

                <div class="border-l-4 border-accent-500 pl-4 bg-accent-50/30 p-4 rounded-r-lg mb-4">
                  <p class="text-sm text-nautical-700 mb-2">
                    <strong class="text-primary-800">Evolution:</strong> We didn't start here. Initial architecture used fat repositories mixing service logic with data access. Straightforward but not testable.
                  </p>
                  <p class="text-sm text-nautical-700">
                    <strong class="text-primary-800">Problem:</strong> Couldn't test business logic without spinning up a database. Every test was an integration test, slow and harder to debug.
                  </p>
                  <p class="text-sm text-nautical-700">
                    <strong class="text-primary-800">Solution:</strong> Refactored to 3-layer architecture with dependency injection. TDD revealed the problem early, and the refactor proved its value immediately. Unit tests ran in seconds instead of minutes.
                  </p>
                </div>

                <p class="text-nautical-800 leading-relaxed mb-4">
                  Clean separation of concerns with testability via dependency injection. This pattern made working with Claude dramatically more efficient. We could validate generated code quickly through fast unit tests, catching issues before they became integration problems.
                </p>

                <div class="code-block">
                  <div class="text-sm font-mono">
                    <div class="mb-2"><span class="text-primary-700 font-semibold">API Layer</span> (routes/)</div>
                    <div class="pl-4 text-nautical-700 mb-3">
                      • HTTP request/response handling<br/>
                      • Middleware application (auth, RBAC)<br/>
                      • Route scoping: public / protected / admin<br/>
                      • Minimal business logic
                    </div>

                    <div class="mb-2"><span class="text-primary-700 font-semibold">Service Layer</span> (services/)</div>
                    <div class="pl-4 text-nautical-700 mb-3">
                      • Business logic and orchestration<br/>
                      • Depends on repository traits (not concrete implementations)<br/>
                      • Modular design: Split large services into focused modules<br/>
                      • Error handling and validation
                    </div>

                    <div class="mb-2"><span class="text-primary-700 font-semibold">Repository Layer</span> (repositories/)</div>
                    <div class="pl-4 text-nautical-700">
                      • Data access abstraction<br/>
                      • Trait-based design for testability<br/>
                      • PostgreSQL implementations for production<br/>
                      • Mock implementations for unit testing
                    </div>
                  </div>
                </div>

                <p class="text-sm text-nautical-600 italic mt-3">
                  <strong>Why this matters:</strong> Services testable with mock repositories (no database needed). Changes to data layer don't cascade through the system. Each layer has single responsibility. Can swap implementations easily.
                </p>
              </div>

              <div class="mb-6">
                <h3 class="subsection-heading">Hybrid API Patterns</h3>
                <p class="text-nautical-800 leading-relaxed mb-4">
                  Two distinct API patterns optimized for different use cases:
                </p>

                <div class="grid md:grid-cols-2 gap-4">
                  <div class="border-2 border-primary-300 rounded-lg p-4 bg-primary-50/50">
                    <h4 class="font-bold text-primary-800 mb-2">/api/* (SSR Proxy)</h4>
                    <ul class="text-sm text-nautical-700 space-y-1">
                      <li>• Server-side data fetching for initial page loads</li>
                      <li>• Session-based authentication handling</li>
                      <li>• Nuxt server acts as proxy to backend</li>
                      <li>• <strong>Why:</strong> SEO, security (httpOnly cookies), UX</li>
                    </ul>
                  </div>
                  <div class="border-2 border-accent-300 rounded-lg p-4 bg-accent-50/50">
                    <h4 class="font-bold text-primary-800 mb-2">/backend/* (Direct Rust)</h4>
                    <ul class="text-sm text-nautical-700 space-y-1">
                      <li>• Client-side mutations and API calls</li>
                      <li>• Stateless Rust API for maximum throughput</li>
                      <li>• Direct backend communication</li>
                      <li>• <strong>Why:</strong> Performance, real-time updates</li>
                    </ul>
                  </div>
                </div>

                <!-- Architecture Diagram -->
                <div class="mt-8">
                  <h4 class="text-lg font-bold text-primary-800 mb-4 text-center">Request Flow Architecture</h4>

                  <!-- Desktop: Show diagram -->
                  <div class="hidden md:block">
                    <MermaidDiagram :diagram="architectureDiagram" />
                  </div>

                  <!-- Mobile: Show message -->
                  <div class="block md:hidden bg-gradient-to-br from-nautical-50 to-primary-50 border-2 border-nautical-300 rounded-lg p-6 text-center">
                    <p class="text-sm text-nautical-700 mb-2">
                      📱 Architecture diagram available on larger screens
                    </p>
                    <p class="text-xs text-nautical-600">
                      View this page on a tablet or desktop to see the interactive request flow diagram
                    </p>
                  </div>

                  <p class="text-xs text-nautical-600 italic text-center mt-4">
                    <strong>Four distinct pathways:</strong> ① Built-in Session (Green) - nuxt-auth-utils automatic route reads cookie, no backend/DB | ② Custom Auth Routes (Steel) - Login/register/profile hit Rust backend and update session | ③ SSR (Blue) - Gets session + fetches page data from backend | ④ Direct Backend (Cyan dashed) - Client-side data operations bypass Nuxt.<br/>
                    * All HTTP traffic reverse-proxied through Nginx (SSL termination, routing). Pathways ②③④ share single Rust → PostgreSQL connection.
                  </p>
                </div>
              </div>

              <div>
                <h3 class="subsection-heading">Service Container Pattern</h3>
                <p class="text-nautical-800 leading-relaxed">
                  Centralized dependency injection managing service lifecycle and configuration.
                  Supports environment-specific setup (development, testing, production) with
                  appropriate repository implementations and background task configuration.
                </p>
              </div>
            </section>

            <!-- Section 3: Technology Stack -->
            <section id="tech-stack" class="mb-12 scroll-mt-24">
              <h2 class="section-heading">Technology Stack</h2>

              <div class="mb-8">
                <h3 class="subsection-heading">Backend Stack</h3>

                <!-- Backend Badges -->
                <div class="flex flex-wrap gap-3 mb-4">
                  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" class="tech-badge" />
                  <img src="https://img.shields.io/badge/Actix--Web-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Actix-Web" class="tech-badge" />
                  <img src="https://img.shields.io/badge/PostgreSQL-4169E1?style=for-the-badge&logo=postgresql&logoColor=white" alt="PostgreSQL" class="tech-badge" />
                  <img src="https://img.shields.io/badge/SQLx-000000?style=for-the-badge&logo=rust&logoColor=white" alt="SQLx" class="tech-badge" />
                </div>

                <ul class="space-y-3">
                  <li class="tech-item">
                    <strong class="text-primary-700">Rust 1.90.0</strong> -
                    <span class="text-nautical-700">Type safety, performance, fearless concurrency. Compile-time guarantees catch errors early, valuable when learning new patterns with Claude.</span>
                  </li>
                  <li class="tech-item">
                    <strong class="text-primary-700">Actix-web 4.x</strong> -
                    <span class="text-nautical-700">High-performance async framework handling high throughput with minimal resources, perfect for 2GB RAM deployment constraint.</span>
                  </li>
                  <li class="tech-item">
                    <strong class="text-primary-700">PostgreSQL 17 + SQLx</strong> -
                    <span class="text-nautical-700">Compile-time query verification prevents SQL errors. Using pre-built image (<code class="bg-nautical-100 px-1 text-xs">ghcr.io/fboulnois/pg_uuidv7</code>) with UUIDv7 extension. Full-text search, JSONB flexibility, ACID compliance.</span>
                  </li>
                  <li class="tech-item">
                    <strong class="text-primary-700">JWT + bcrypt</strong> -
                    <span class="text-nautical-700">Stateless authentication with bcrypt password hashing (cost 12) and refresh token rotation.</span>
                  </li>
                </ul>
              </div>

              <div class="mb-8">
                <h3 class="subsection-heading">Frontend Stack</h3>

                <!-- Frontend Badges -->
                <div class="flex flex-wrap gap-3 mb-4">
                  <img src="https://img.shields.io/badge/Nuxt.js-00DC82?style=for-the-badge&logo=nuxtdotjs&logoColor=white" alt="Nuxt.js" class="tech-badge" />
                  <img src="https://img.shields.io/badge/Vue.js-4FC08D?style=for-the-badge&logo=vuedotjs&logoColor=white" alt="Vue.js" class="tech-badge" />
                  <img src="https://img.shields.io/badge/TypeScript-3178C6?style=for-the-badge&logo=typescript&logoColor=white" alt="TypeScript" class="tech-badge" />
                  <img src="https://img.shields.io/badge/TailwindCSS-06B6D4?style=for-the-badge&logo=tailwindcss&logoColor=white" alt="TailwindCSS" class="tech-badge" />
                </div>

                <ul class="space-y-3">
                  <li class="tech-item">
                    <strong class="text-primary-700">Nuxt.js 4.0.3</strong> -
                    <span class="text-nautical-700">Vue 3 with SSR for SEO. Search engines receive server-rendered HTML for portfolio content indexing.</span>
                  </li>
                  <li class="tech-item">
                    <strong class="text-primary-700">TypeScript</strong> -
                    <span class="text-nautical-700">Type safety across full stack. Compile-time error catching and better IDE support.</span>
                  </li>
                  <li class="tech-item">
                    <strong class="text-primary-700">TailwindCSS</strong> -
                    <span class="text-nautical-700">Utility-first CSS with custom nautical theme and responsive design system.</span>
                  </li>
                  <li class="tech-item">
                    <strong class="text-primary-700">Pinia + Action Composables</strong> -
                    <span class="text-nautical-700">State management with action composable pattern. Pure stores, testable composables, separation of concerns.</span>
                  </li>
                </ul>
              </div>

              <div>
                <h3 class="subsection-heading">Infrastructure</h3>

                <!-- Infrastructure Badges -->
                <div class="flex flex-wrap gap-3 mb-4">
                  <img src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white" alt="Docker" class="tech-badge" />
                  <img src="https://img.shields.io/badge/Nginx-009639?style=for-the-badge&logo=nginx&logoColor=white" alt="Nginx" class="tech-badge" />
                  <img src="https://img.shields.io/badge/AWS-232F3E?style=for-the-badge&logo=amazonaws&logoColor=white" alt="AWS" class="tech-badge" />
                </div>

                <ul class="space-y-3">
                  <li class="tech-item">
                    <strong class="text-primary-700">Docker Compose</strong> -
                    <span class="text-nautical-700">Container orchestration for development and production. Consistent environments, easy deployment, service isolation.</span>
                  </li>
                  <li class="tech-item">
                    <strong class="text-primary-700">Nginx</strong> -
                    <span class="text-nautical-700">Reverse proxy with SSL termination. Static file serving, request routing to Nuxt/Rust, performance optimization.</span>
                  </li>
                  <li class="tech-item">
                    <strong class="text-primary-700">AWS t3.small (2GB RAM)</strong> -
                    <span class="text-nautical-700">Production deployment with resource constraints driving optimization decisions. SQLx connection pooling with sensible defaults, PostgreSQL tuned for 2GB environment, stateless API design.</span>
                  </li>
                </ul>
              </div>
            </section>

            <!-- Section 4: Key Features -->
            <section id="features" class="mb-12 scroll-mt-24">
              <h2 class="section-heading">Key Features</h2>

              <div class="space-y-6">
                <div class="feature-highlight">
                  <h3 class="feature-highlight-title">Authentication & Security</h3>
                  <p class="text-sm text-nautical-700 mb-3">
                    <strong class="text-primary-800">User Need:</strong> Account creation with email/password or Google, email verification to unlock features, password reset when forgotten, self-service account deletion.
                  </p>
                  <p class="text-sm text-nautical-700 mb-4">
                    <strong class="text-primary-800">Technical Implementation:</strong>
                  </p>
                  <ul class="feature-list">
                    <li><strong>Google OAuth with PKCE:</strong> Proof Key for Code Exchange for enhanced security (architected for multi-provider support)</li>
                    <li><strong>Email verification:</strong> Secure token-based verification (24-hour single-use tokens) required for trusted operations</li>
                    <li><strong>Password reset flow:</strong> AWS SES integration with secure tokens and email delivery tracking</li>
                    <li><strong>Account management:</strong> Self-service deletion with immediate permanent removal (GDPR/CCPA compliant)</li>
                  </ul>
                </div>

                <div class="feature-highlight">
                  <h3 class="feature-highlight-title">Role-Based Access Control (RBAC)</h3>
                  <p class="text-sm text-nautical-700 mb-3">
                    <strong class="text-primary-800">User Need:</strong> Control access to features and content. Email verification prevents spam accounts from creating timers/phrases. Trusted contacts can view personal testimony content. Admin moderates phrases and manages users.
                  </p>
                  <p class="text-sm text-nautical-700 mb-4">
                    <strong class="text-primary-800">Technical Implementation:</strong>
                  </p>
                  <ul class="feature-list">
                    <li><strong>Feature-based role system:</strong> <code class="bg-nautical-100 px-1 text-xs">user</code> (read-only base), <code class="bg-nautical-100 px-1 text-xs">email-verified</code> (create timers/suggestions), <code class="bg-nautical-100 px-1 text-xs">trusted-contact</code> (view personal content), <code class="bg-nautical-100 px-1 text-xs">admin</code> (full system control)</li>
                    <li><strong>Middleware-based authorization:</strong> Applied at route scope level for clear security boundaries</li>
                    <li><strong>Route scoping:</strong> Public, protected, and admin routes with automatic permission enforcement</li>
                    <li><strong>Flexible role management:</strong> Admin can grant/revoke manageable roles (preserves immutable base role)</li>
                  </ul>
                </div>

                <div class="feature-highlight">
                  <h3 class="feature-highlight-title">Domain Events & Notifications</h3>
                  <p class="text-sm text-nautical-700 mb-3">
                    <strong class="text-primary-800">User Flow:</strong> User suggests phrase → System fires domain event → Admin receives email notification → Admin approves/rejects → System fires approval event → User receives confirmation email. Also handles security alerts and access request notifications.
                  </p>
                  <p class="text-sm text-nautical-700 mb-4">
                    <strong class="text-primary-800">Technical Implementation:</strong>
                  </p>
                  <ul class="feature-list">
                    <li><strong>Event bus with observer pattern:</strong> Type-safe event routing via TypeId, fire-and-forget async execution</li>
                    <li><strong>Email notification system:</strong> Security alerts, phrase suggestion approvals, access request notifications</li>
                    <li><strong>AWS SES integration:</strong> HTML email templates with bounce/complaint tracking and suppression list</li>
                    <li><strong>Concurrent handler execution:</strong> Semaphore-based backpressure, error isolation (one handler failure doesn't affect others)</li>
                  </ul>
                </div>

                <div class="feature-highlight">
                  <h3 class="feature-highlight-title">Admin Panel</h3>
                  <p class="text-sm text-nautical-700 mb-3">
                    <strong class="text-primary-800">User Need:</strong> Admin (me) needs to moderate phrase suggestions, grant access requests from trusted contacts, manage user accounts, view system statistics, and handle email deliverability issues.
                  </p>
                  <p class="text-sm text-nautical-700 mb-4">
                    <strong class="text-primary-800">Technical Implementation:</strong>
                  </p>
                  <ul class="feature-list">
                    <li><strong>User management:</strong> Account activation/deactivation, role assignment, password reset, promote to admin</li>
                    <li><strong>Phrase moderation:</strong> Approval workflow for user-submitted motivational phrases with admin review</li>
                    <li><strong>Access request handling:</strong> Review and approve/reject trusted-contact role requests with email notifications</li>
                    <li><strong>System dashboard:</strong> Statistics, user activity, content moderation queue</li>
                    <li><strong>Email suppression:</strong> Bounce and complaint handling for deliverability compliance</li>
                  </ul>
                </div>

                <div class="feature-highlight">
                  <h3 class="feature-highlight-title">Data Privacy & Compliance</h3>
                  <p class="text-sm text-nautical-700 mb-3">
                    <strong class="text-primary-800">User Need:</strong> Users need control over their data. Self-service account deletion when leaving. Data export for portability. Privacy-respecting design without invasive tracking.
                  </p>
                  <p class="text-sm text-nautical-700 mb-4">
                    <strong class="text-primary-800">Technical Implementation:</strong>
                  </p>
                  <ul class="feature-list">
                    <li><strong>GDPR/CCPA compliance:</strong> Self-service account deletion (immediate permanent removal), data export in JSON format</li>
                    <li><strong>Normalized auth schema:</strong> 5-table architecture (users, credentials, external_logins, profiles, preferences) with clear data boundaries</li>
                    <li><strong>Email suppression list:</strong> AWS SES compliance for bounce and complaint handling</li>
                    <li><strong>Privacy-first design:</strong> No analytics tracking, no third-party cookies, minimal data collection</li>
                  </ul>
                </div>
              </div>
            </section>

            <!-- Section 5: Security & Compliance -->
            <section id="security" class="mb-12 scroll-mt-24">
              <h2 class="section-heading">Security & Compliance</h2>

              <div class="grid md:grid-cols-2 gap-6">
                <div class="security-card">
                  <h3 class="security-title">JWT + Refresh Token Hybrid</h3>
                  <p class="text-sm text-nautical-700 mb-2">
                    Short-lived access tokens (1 hour) with long-lived refresh tokens (1 week rolling, 6 month hard limit).
                  </p>
                  <ul class="text-sm text-nautical-600 space-y-1">
                    <li>• Stateless JWT for performance (no DB lookup per request)</li>
                    <li>• Revocable refresh tokens (database-backed)</li>
                    <li>• Token rotation on every refresh (detects theft)</li>
                  </ul>
                </div>

                <div class="security-card">
                  <h3 class="security-title">bcrypt Password Hashing</h3>
                  <p class="text-sm text-nautical-700 mb-2">
                    Industry-standard password hashing with cost factor 12.
                  </p>
                  <ul class="text-sm text-nautical-600 space-y-1">
                    <li>• Slow by design (brute force resistant)</li>
                    <li>• Adaptive cost (can increase over time)</li>
                    <li>• Battle-tested, widely supported</li>
                  </ul>
                </div>

                <div class="security-card">
                  <h3 class="security-title">OAuth with PKCE</h3>
                  <p class="text-sm text-nautical-700 mb-2">
                    Authorization Code flow with PKCE (Proof Key for Code Exchange).
                  </p>
                  <ul class="text-sm text-nautical-600 space-y-1">
                    <li>• Prevents authorization code interception</li>
                    <li>• Required for public clients (SPAs)</li>
                    <li>• More secure than implicit flow</li>
                  </ul>
                </div>

                <div class="security-card">
                  <h3 class="security-title">Email Verification</h3>
                  <p class="text-sm text-nautical-700 mb-2">
                    Token-based verification required for trusted operations.
                  </p>
                  <ul class="text-sm text-nautical-600 space-y-1">
                    <li>• Secure random tokens (24-hour expiration)</li>
                    <li>• Single-use tokens (prevents replay attacks)</li>
                    <li>• Prevents spam accounts, confirms identity</li>
                  </ul>
                </div>

                <div class="security-card">
                  <h3 class="security-title">HTTPS/TLS Encryption</h3>
                  <p class="text-sm text-nautical-700 mb-2">
                    All connections encrypted with SSL termination at Nginx.
                  </p>
                  <ul class="text-sm text-nautical-600 space-y-1">
                    <li>• SSL certificates for production domain</li>
                    <li>• Self-signed certs for local development</li>
                    <li>• HTTP-only secure cookies for auth tokens</li>
                  </ul>
                </div>

                <div class="security-card">
                  <h3 class="security-title">GDPR/CCPA Compliance</h3>
                  <p class="text-sm text-nautical-700 mb-2">
                    Data privacy compliance with user rights enforcement.
                  </p>
                  <ul class="text-sm text-nautical-600 space-y-1">
                    <li>• Immediate account deletion (no grace period)</li>
                    <li>• Data export in machine-readable JSON format</li>
                    <li>• Clear data boundaries in normalized schema</li>
                  </ul>
                </div>
              </div>
            </section>

            <!-- Section 6: Testing Strategy -->
            <section id="testing" class="mb-12 scroll-mt-24">
              <h2 class="section-heading">Testing Strategy</h2>

              <div class="mb-6">
                <p class="text-nautical-800 leading-relaxed mb-4">
                  Comprehensive test coverage (~600 tests total) across all layers with paradigm-based testing approach.
                  TDD workflow with Claude: write tests defining behavior, Claude implements, validate against specification. Fast feedback loop crucial for learning new patterns.
                </p>
              </div>

              <!-- Testing Strategy Diagram -->
              <div class="mt-8 mb-8">
                <h4 class="text-lg font-bold text-primary-800 mb-4 text-center">Testing Pyramid: ~600 Tests Across All Layers</h4>

                <!-- Desktop: Show diagram -->
                <div class="hidden md:block">
                  <MermaidDiagram :diagram="testingStrategyDiagram" />
                </div>

                <!-- Mobile: Show message -->
                <div class="block md:hidden bg-gradient-to-br from-nautical-50 to-primary-50 border-2 border-nautical-300 rounded-lg p-6 text-center">
                  <p class="text-sm text-nautical-700 mb-2">
                    📱 Testing diagram available on larger screens
                  </p>
                  <p class="text-xs text-nautical-600">
                    View this page on a tablet or desktop to see the testing pyramid visualization
                  </p>
                </div>

                <p class="text-xs text-nautical-600 italic text-center mt-4">
                  <strong>Paradigm-Based Testing:</strong> Each layer tested at the appropriate level. Repository tests verify database operations with real PostgreSQL. Service tests verify business logic with mocked data. API tests verify HTTP behavior and integration. Frontend tests verify components, composables, and utilities.
                </p>
              </div>

              <div class="grid md:grid-cols-3 gap-6 mb-6">
                <div class="test-card">
                  <div class="text-2xl font-bold text-primary-600 mb-2">Repository Layer</div>
                  <p class="text-sm text-nautical-700 mb-3">Integration tests with testcontainers</p>
                  <ul class="text-xs text-nautical-600 space-y-1">
                    <li>• Real PostgreSQL database in Docker</li>
                    <li>• Tests actual SQL queries and transactions</li>
                    <li>• Verifies database constraints and indexes</li>
                  </ul>
                </div>

                <div class="test-card">
                  <div class="text-2xl font-bold text-primary-600 mb-2">Service Layer</div>
                  <p class="text-sm text-nautical-700 mb-3">Unit tests with mock repositories</p>
                  <ul class="text-xs text-nautical-600 space-y-1">
                    <li>• Fast execution, no database required</li>
                    <li>• Tests business logic in isolation</li>
                    <li>• Mockall for trait-based mocking</li>
                  </ul>
                </div>

                <div class="test-card">
                  <div class="text-2xl font-bold text-primary-600 mb-2">API Layer</div>
                  <p class="text-sm text-nautical-700 mb-3">End-to-end tests with actix-test</p>
                  <ul class="text-xs text-nautical-600 space-y-1">
                    <li>• Full HTTP request/response cycle</li>
                    <li>• Tests middleware and route handlers</li>
                    <li>• Verifies authentication and authorization</li>
                  </ul>
                </div>
              </div>

              <div class="border-l-4 border-primary-500 pl-4 bg-primary-50 p-4 rounded-r-lg">
                <p class="text-sm text-nautical-700 mb-2">
                  <strong class="text-primary-800">Frontend Testing:</strong> Component tests, action composable tests, pure service tests, and utilities.
                  175+ tests achieving 100% success rate with comprehensive coverage of action composables, pure stores, and services.
                </p>
                <p class="text-xs text-nautical-600 italic">
                  Testing philosophy: Each layer tested at appropriate level. Repository tests verify database operations.
                  Service tests verify business logic with mocked data. API tests verify HTTP behavior and integration.
                </p>
              </div>
            </section>

            <!-- Section 7: Infrastructure -->
            <section id="infrastructure" class="mb-12 scroll-mt-24">
              <h2 class="section-heading">Infrastructure & Deployment</h2>

              <div class="mb-6">
                <h3 class="subsection-heading">Production Deployment</h3>
                <p class="text-nautical-800 leading-relaxed mb-4">
                  Live at <a href="https://kennwilliamson.org" class="text-primary-600 hover:text-primary-800 underline font-semibold">kennwilliamson.org</a> on AWS t3.small instance (2GB RAM).
                  All resource allocation decisions driven by memory constraints.
                </p>

                <div class="grid md:grid-cols-2 gap-4">
                  <div class="infra-card">
                    <h4 class="font-bold text-primary-700 mb-2">Docker Compose</h4>
                    <p class="text-sm text-nautical-700">Container orchestration for 4 services: Nginx, Nuxt.js, Rust backend, PostgreSQL. Persistent volumes for database, automated health checks, environment-specific configurations.</p>
                  </div>
                  <div class="infra-card">
                    <h4 class="font-bold text-primary-700 mb-2">Nginx Reverse Proxy</h4>
                    <p class="text-sm text-nautical-700">SSL termination, static file serving, request routing. Serves frontend assets, proxies API requests to Nuxt/Rust, gzip compression for performance.</p>
                  </div>
                  <div class="infra-card">
                    <h4 class="font-bold text-primary-700 mb-2">Resource Optimization</h4>
                    <p class="text-sm text-nautical-700">SQLx connection pooling with sensible defaults, PostgreSQL tuned for 2GB RAM, stateless Rust API design enables horizontal scaling without memory penalties.</p>
                  </div>
                  <div class="infra-card">
                    <h4 class="font-bold text-primary-700 mb-2">AWS SES Integration</h4>
                    <p class="text-sm text-nautical-700">Email delivery with bounce/complaint tracking, HTML email templates, suppression list management for deliverability compliance.</p>
                  </div>
                </div>
              </div>

              <div>
                <h3 class="subsection-heading">Development Workflow</h3>
                <p class="text-nautical-800 leading-relaxed mb-4">
                  Automated setup scripts for consistent development environment with hot reload for both frontend and backend.
                </p>

                <div class="code-block">
                  <div class="text-sm font-mono text-nautical-700">
                    <div class="mb-1">./scripts/dev-start.sh &nbsp;&nbsp;&nbsp;# Start all services with hot reload</div>
                    <div class="mb-1">./scripts/health-check.sh # Verify service health</div>
                    <div class="mb-1">./scripts/setup-db.sh &nbsp;&nbsp;&nbsp;&nbsp;# Run database migrations</div>
                    <div class="mb-1">./scripts/prepare-sqlx.sh # SQLx compile-time verification</div>
                  </div>
                </div>
              </div>
            </section>

            <!-- Section 8: Course Corrections & What We Learned -->
            <section id="course-corrections" class="scroll-mt-24">
              <h2 class="section-heading">Course Corrections & What We Learned</h2>

              <p class="text-nautical-800 leading-relaxed mb-6">
                Building a production application while learning two new technologies with IA meant iteration, mistakes, and course corrections. Here are the most instructive examples of what went wrong, how we fixed it, and what it taught us about practicing Intelligence Augmentation.
              </p>

              <div class="space-y-6">
                <!-- Template Recursion Story -->
                <div class="challenge-card">
                  <h3 class="challenge-title">The Great Template Recursion Rabbit Hole</h3>
                  <p class="challenge-text mb-3">
                    <strong>The Problem:</strong> Email templates built with Askama (compile-time templating) threw infinite recursion errors. After a dozen+ debugging attempts and web searches, we concluded Askama wasn't production-ready and switched the entire implementation to minijinja (runtime templating).
                  </p>
                  <p class="challenge-text mb-3">
                    <strong>The Plot Twist:</strong> Minijinja gave us the exact same infinite recursion error, but at runtime during tests, not compile time. That's when we finally spotted it: one template was importing itself. Nothing wrong with Askama. Just a bug Claude missed despite reviewing the template multiple times.
                  </p>
                  <div class="solution-box">
                    <strong class="text-primary-700">Lesson:</strong> GMs can confidently miss obvious bugs even after multiple reviews. Better error messages (runtime vs compile-time) sometimes reveal what repeated analysis doesn't. IA requires human validation even when the GM seems confident. We switched back to Askama.
                  </div>
                </div>

                <!-- Constructor vs Builder Pattern -->
                <div class="challenge-card">
                  <h3 class="challenge-title">OOP Instincts Meet Rust Idioms</h3>
                  <p class="challenge-text mb-3">
                    <strong>The Problem:</strong> Initial implementation used multiple constructors: <code class="bg-nautical-100 px-1">new()</code>, <code class="bg-nautical-100 px-1">new_with_oauth()</code>, <code class="bg-nautical-100 px-1">new_with_xyz()</code>. Coming from C# where multiple constructors are normal, this felt natural. Claude didn't push back. It seemed reasonable.
                  </p>
                  <p class="challenge-text mb-3">
                    <strong>The Discovery:</strong> Learning Rust idioms, we discovered builder pattern is heavily favored. Refactored everything to builders. Cleaner, more flexible, more idiomatic Rust.
                  </p>
                  <div class="solution-box">
                    <strong class="text-primary-700">Lesson:</strong> GMs can follow your lead even when you're wrong. Need to validate against language/framework best practices independently. Your OOP background doesn't automatically translate to every paradigm.
                  </div>
                </div>

                <!-- OAuth Architecture Pivot -->
                <div class="challenge-card">
                  <h3 class="challenge-title">OAuth Architecture: SPA → Redis-Backed PKCE</h3>
                  <p class="challenge-text mb-3">
                    <strong>Initial Approach:</strong> Started with SPA-style OAuth flow. Seemed straightforward, matched common patterns.
                  </p>
                  <p class="challenge-text mb-3">
                    <strong>The Realization:</strong> Our architecture required auth to come from the Rust server, not the Nuxt server or client. Couldn't "trust" the frontend with OAuth secrets. SPA pattern wouldn't work.
                  </p>
                  <p class="challenge-text mb-3">
                    <strong>The Solution:</strong> Added Redis for stateful PKCE token management (we already had it for rate limiting). Refactored entire OAuth flow for server-side security.
                  </p>
                  <div class="solution-box">
                    <strong class="text-primary-700">Lesson:</strong> Security requirements can force architecture changes. Early prototypes don't always reveal security constraints. Sometimes you need to build it to understand what's required.
                  </div>
                </div>

                <!-- Session Management -->
                <div class="challenge-card">
                  <h3 class="challenge-title">Session Management: The Mental Model Gap</h3>
                  <p class="challenge-text mb-3">
                    <strong>The Challenge:</strong> Updating user profiles, display names, and preferences required keeping session state synchronized. I wanted passthrough routes where updates go through Nuxt server to update the session, making session the source of truth.
                  </p>
                  <p class="challenge-text mb-3">
                    <strong>What Claude Kept Suggesting:</strong> Parallel session + store architecture. Session has auth, store has profile, sync them when either changes. Multiple prompt iterations couldn't get past this pattern.
                  </p>
                  <p class="challenge-text mb-3">
                    <strong>The Breakthrough:</strong> Eventually got to the simpler approach: session as store, passthrough routes, single source of truth. No sync complexity.
                  </p>
                  <div class="solution-box">
                    <strong class="text-primary-700">Lesson:</strong> Sometimes the simpler solution requires patient iteration. GMs can get stuck on a pattern even when you're describing something different. This is where variability in Claude's performance shows: autonomous on large features, stuck on conceptual pivots.
                  </div>
                </div>

                <!-- PostgreSQL UUIDv7 -->
                <div class="challenge-card">
                  <h3 class="challenge-title">PostgreSQL UUIDv7: When to Choose Pragmatism</h3>
                  <p class="challenge-text mb-3">
                    <strong>The Goal:</strong> Use UUIDv7 primary keys (time-ordered, better indexing than random UUIDs).
                  </p>
                  <p class="challenge-text mb-3">
                    <strong>The Struggle:</strong> Couldn't get pg_uuidv7 extension installed on vanilla PostgreSQL 17 Docker image. Tried multiple approaches with Claude, researched Docker multi-stage builds, fought with PostgreSQL extension compilation.
                  </p>
                  <p class="challenge-text mb-3">
                    <strong>The Pragmatic Choice:</strong> Used pre-built image (<code class="bg-nautical-100 px-1">ghcr.io/fboulnois/pg_uuidv7:1.6.0</code>) with extension already compiled. Moved on.
                  </p>
                  <div class="solution-box">
                    <strong class="text-primary-700">Lesson:</strong> Sometimes pragmatic choice beats perfect solution. Don't let the perfect become the enemy of the shipped. The pre-built image works perfectly. Solving the Docker compilation rabbit hole wouldn't have added value.
                  </div>
                </div>

                <!-- SSR Data Loading -->
                <div class="challenge-card">
                  <h3 class="challenge-title">Confidently Investigating the Wrong Problem</h3>
                  <p class="challenge-text mb-3">
                    <strong>The Bug:</strong> Public timer list wasn't loading for authenticated users. Simple SSR data loading issue.
                  </p>
                  <p class="challenge-text mb-3">
                    <strong>Claude's Investigation:</strong> Spent 20 minutes confidently debugging JWT tokens, SSR hydration patterns, and Pinia state management. Even started editing the wrong component (user timer history instead of public timer list).
                  </p>
                  <p class="challenge-text mb-3">
                    <strong>The Fix:</strong> One line: <code class="bg-nautical-100 px-1">await useAsyncData('public-timers-list', ...)</code>. Authenticated users just needed to load the public timer data. 30 seconds after human redirection.
                  </p>
                  <div class="solution-box">
                    <strong class="text-primary-700">Lesson:</strong> GMs can be confidently wrong about simple problems, investigating complex solutions when the answer is obvious. Human oversight essential to redirect from rabbit holes. Sometimes "just load the data" beats 20 minutes of SSR hydration analysis.
                  </div>
                </div>

                <!-- Resource Constraints Evolution -->
                <div class="challenge-card">
                  <h3 class="challenge-title">Resource Targeting: 1GB → 2GB</h3>
                  <p class="challenge-text mb-3">
                    <strong>Initial Plan:</strong> Target AWS micro instance (1GB RAM, free tier).
                  </p>
                  <p class="challenge-text mb-3">
                    <strong>The Discovery:</strong> AWS free tier is time-limited, not permanent. Micro instance too constrained for full stack (Nginx, Nuxt, Rust, PostgreSQL).
                  </p>
                  <p class="challenge-text mb-3">
                    <strong>The Adjustment:</strong> Landed on t3.small (2GB RAM) as cost-effective tier. Reasonable price, next tier up is a big jump in cost. All optimization decisions driven by this 2GB constraint.
                  </p>
                  <div class="solution-box">
                    <strong class="text-primary-700">Lesson:</strong> Constraints drive optimization. The 2GB limit forced good architectural decisions: stateless API design, efficient connection pooling, PostgreSQL tuning. Constraints can improve design by preventing over-engineering.
                  </div>
                </div>
              </div>
            </section>

            <!-- Section 9: What This Demonstrates About IA -->
            <section id="ia-demonstration" class="mb-12 scroll-mt-24">
              <h2 class="section-heading">What This Demonstrates About IA</h2>

              <p class="text-nautical-800 leading-relaxed mb-6">
                This project validates the Intelligence Augmentation framework I evangelize professionally. It's not just a portfolio piece. It's proof that the IA approach works for complex learning in production environments.
              </p>

              <div class="mb-6">
                <h3 class="subsection-heading">The Proof Points</h3>
                <div class="grid md:grid-cols-3 gap-4">
                  <div class="border-2 border-primary-300 rounded-lg p-4 bg-white">
                    <h4 class="font-bold text-primary-700 mb-2">Speed</h4>
                    <p class="text-sm text-nautical-700">Two months part-time from zero Rust/Nuxt knowledge to production deployment. Would have taken 6+ months in the pre-GM era.</p>
                  </div>
                  <div class="border-2 border-primary-300 rounded-lg p-4 bg-white">
                    <h4 class="font-bold text-primary-700 mb-2">Quality</h4>
                    <p class="text-sm text-nautical-700">600+ tests across all layers. Clean 3-layer architecture. Live at kennwilliamson.org with real infrastructure, not toy project.</p>
                  </div>
                  <div class="border-2 border-primary-300 rounded-lg p-4 bg-white">
                    <h4 class="font-bold text-primary-700 mb-2">Learning</h4>
                    <p class="text-sm text-nautical-700">Genuine understanding of Rust (ownership, lifetimes, traits) and Nuxt (SSR hydration, composition API). IA accelerated learning, didn't replace it.</p>
                  </div>
                </div>
              </div>

              <div class="mb-6">
                <h3 class="subsection-heading">Why This Is Different from "AI Will Replace Developers"</h3>
                <div class="border-l-4 border-accent-500 pl-4 bg-accent-50/30 p-4 rounded-r-lg">
                  <ul class="space-y-2 text-sm text-nautical-700">
                    <li><strong class="text-primary-800">I made every architectural decision:</strong> 3-layer pattern, hybrid API design, normalized auth schema, security choices. All human judgment calls.</li>
                    <li><strong class="text-primary-800">I caught Claude's mistakes:</strong> Template recursion bug, session management complexity, non-idiomatic Rust patterns. Validation was critical.</li>
                    <li><strong class="text-primary-800">I validated against best practices:</strong> Builder pattern over constructors, TDD for confidence, security requirements driving OAuth changes.</li>
                    <li><strong class="text-primary-800">Claude accelerated, didn't replace:</strong> Generated boilerplate, explored patterns, helped me learn. But I understand what I built.</li>
                  </ul>
                </div>
              </div>

              <div class="mb-6">
                <h3 class="subsection-heading">For Companies Evaluating IA</h3>
                <p class="text-nautical-800 leading-relaxed mb-4">
                  This project demonstrates what your team can accomplish with IA:
                </p>
                <ul class="space-y-3 list-disc list-inside text-nautical-700">
                  <li><strong>Dramatically faster learning curves</strong> - New technologies, frameworks, languages accessible in weeks instead of months</li>
                  <li><strong>But requires strong fundamentals</strong> - Architecture knowledge, testing discipline, security awareness all critical</li>
                  <li><strong>Not autopilot</strong> - Active collaboration with constant validation, course corrections, human judgment</li>
                  <li><strong>Productivity multiplier</strong> - When paired with skilled engineers, IA enables what would've seemed impossible on compressed timelines</li>
                </ul>
              </div>

              <div class="border-l-4 border-primary-500 pl-4 bg-primary-50 p-4 rounded-r-lg">
                <p class="text-sm text-nautical-700 mb-2">
                  <strong class="text-primary-800">The Bottom Line:</strong> This site proves the thesis from my <NuxtLink to="/about/ai" class="text-primary-600 hover:text-primary-800 underline">IA framework</NuxtLink>. Generative models amplify skilled humans, they don't replace human judgment.
                </p>
                <p class="text-xs text-nautical-600 italic">
                  Two months. Part-time. Zero to production. That's what Intelligence Augmentation looks like when practiced effectively.
                </p>
              </div>
            </section>

              </div>
            </div>
          </article>
        </main>
      </div>

      <!-- Call to Action -->
      <div class="text-center mt-12">
        <p class="text-nautical-200 mb-6">
          Interested in working together? Let's connect.
        </p>
        <div class="flex flex-col sm:flex-row gap-4 justify-center">
          <NuxtLink
            to="/about"
            class="px-8 py-3 bg-gradient-to-r from-primary-600 to-accent-600 hover:from-primary-700 hover:to-accent-700 text-white font-semibold rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105"
          >
            Read My Story
          </NuxtLink>
          <a
            href="https://github.com/kenn-williamson/kennwilliamsondotorg"
            target="_blank"
            rel="noopener noreferrer"
            class="px-8 py-3 bg-nautical-700 hover:bg-nautical-600 text-white font-semibold rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105"
          >
            View on GitHub
          </a>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import MermaidDiagram from '~/components/MermaidDiagram.vue'
import { architectureDiagram } from '~/diagrams/architecture.mermaid'
import { iaWorkflowDiagram } from '~/diagrams/ia-workflow.mermaid'
import { testingStrategyDiagram } from '~/diagrams/testing-strategy.mermaid'

const mobileMenuOpen = ref(false)
const activeSection = ref('overview')

// Section navigation structure
const sections = [
  { id: 'overview', title: 'Project Overview' },
  { id: 'ia-laboratory', title: 'The IA Laboratory' },
  { id: 'what-it-does', title: 'What the Site Does' },
  { id: 'architecture', title: 'Architecture' },
  { id: 'tech-stack', title: 'Technology Stack' },
  { id: 'features', title: 'Key Features' },
  { id: 'security', title: 'Security & Compliance' },
  { id: 'testing', title: 'Testing Strategy' },
  { id: 'infrastructure', title: 'Infrastructure' },
  { id: 'course-corrections', title: 'Course Corrections' },
  { id: 'ia-demonstration', title: 'What This Demonstrates' }
]

// Get current section title for mobile dropdown
const currentSection = computed(() => {
  const current = sections.find(s => s.id === activeSection.value)
  return current ? current.title : 'Technical Overview'
})

// Track active section based on scroll position
onMounted(() => {
  const observerOptions = {
    root: null,
    rootMargin: '-100px 0px -66%',
    threshold: 0
  }

  const observerCallback = (entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        activeSection.value = entry.target.id
      }
    })
  }

  const observer = new IntersectionObserver(observerCallback, observerOptions)

  sections.forEach(section => {
    const element = document.getElementById(section.id)
    if (element) {
      observer.observe(element)
    }
  })

  onUnmounted(() => {
    observer.disconnect()
  })
})

useHead({
  title: 'Intelligence Augmentation in Practice',
  meta: [
    {
      name: 'description',
      content: 'How I built KennWilliamson.org in 2 months part-time using Intelligence Augmentation with Claude, learning Rust and Nuxt simultaneously. Complete with course corrections, lessons learned, and proof that IA works.',
    },
  ],
})

// Social media sharing
useSocialShare({
  title: 'Intelligence Augmentation in Practice - Building KennWilliamson.org',
  imageKey: 'project'
})
</script>

<style scoped>
/* Nautical steampunk background - deep navy and slate tones */
.nautical-background {
  background: linear-gradient(
    135deg,
    #0f172a 0%,
    #1e293b 25%,
    #334155 50%,
    #1e293b 75%,
    #0f172a 100%
  );
  min-height: 100vh;
}

/* Mobile menu slide animation */
.menu-slide-enter-active,
.menu-slide-leave-active {
  transition: all 0.3s ease;
}

.menu-slide-enter-from,
.menu-slide-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

/* Smooth scroll behavior */
html {
  scroll-behavior: smooth;
}

/* Scroll margin for anchor links (accounts for sticky header) */
.scroll-mt-24 {
  scroll-margin-top: 6rem;
}

/* Main content card */
.content-card {
  @apply bg-gradient-to-br from-nautical-50 via-primary-50 to-sky-50 border-2 border-primary-700 rounded-lg shadow-xl overflow-hidden;
}

/* Metric cards */
.metric-card {
  @apply bg-gradient-to-br from-nautical-700 to-primary-800 border-2 border-nautical-500 rounded-lg p-4 text-center shadow-lg;
}

/* Typography */
.section-heading {
  @apply text-3xl font-bold text-primary-900 mb-6 pb-3 border-b-2 border-primary-300;
}

.subsection-heading {
  @apply text-2xl font-bold text-primary-800 mb-4;
}

/* Prose custom styles */
.prose-custom {
  @apply max-w-none;
}

/* Feature grid */
.feature-grid {
  @apply grid md:grid-cols-2 gap-6 mt-6;
}

.feature-item {
  @apply border-2 border-primary-200 rounded-lg p-5 bg-white shadow-sm hover:shadow-md transition-shadow;
}

.feature-title {
  @apply text-lg font-bold text-primary-700 mb-2;
}

.feature-text {
  @apply text-sm text-nautical-700 leading-relaxed;
}

/* Code blocks */
.code-block {
  @apply bg-nautical-100 border-2 border-nautical-300 rounded-lg p-5 my-4;
}

/* Feature highlights */
.feature-highlight {
  @apply border-l-4 border-accent-500 pl-5 bg-accent-50/30 p-4 rounded-r-lg;
}

.feature-highlight-title {
  @apply text-xl font-bold text-primary-800 mb-3;
}

.feature-list {
  @apply space-y-2 text-sm text-nautical-700;
}

.feature-list li {
  @apply leading-relaxed;
}

/* Security cards */
.security-card {
  @apply border-2 border-primary-300 rounded-lg p-4 bg-gradient-to-br from-white to-primary-50;
}

.security-title {
  @apply text-lg font-bold text-primary-800 mb-2;
}

/* Test cards */
.test-card {
  @apply border-2 border-primary-200 rounded-lg p-5 bg-white hover:border-primary-400 transition-colors;
}

/* Infrastructure cards */
.infra-card {
  @apply bg-gradient-to-br from-nautical-50 to-primary-50 border border-nautical-300 rounded-lg p-4;
}

/* Challenge cards */
.challenge-card {
  @apply bg-white border-2 border-nautical-300 rounded-lg p-6 shadow-sm;
}

.challenge-title {
  @apply text-xl font-bold text-primary-800 mb-3;
}

.challenge-text {
  @apply text-nautical-700 leading-relaxed;
}

.solution-box {
  @apply bg-gradient-to-r from-primary-50 to-accent-50 border-l-4 border-primary-600 pl-4 py-3 rounded-r text-sm text-nautical-700;
}

/* Tech items */
.tech-item {
  @apply leading-relaxed;
}

/* Tech badges - make shields larger and more readable */
.tech-badge {
  height: 32px;
  width: auto;
  transition: transform 0.2s ease;
}

.tech-badge:hover {
  transform: scale(1.05);
}
</style>
