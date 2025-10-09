# AI & The Future of Work

*[Hero image: Something representing human-AI collaboration, maybe a developer at work with AI tools]*

There are two camps when it comes to AI in software development, and I think they're both missing the real story. The evangelists promise AI will replace programmers tomorrow. The skeptics dismiss it as worthless hype. The truth is more nuanced—and more interesting. I call my position "AI realism"—the middle path that actually helps people navigate what's happening.

## The Two Dominant Narratives (Both Wrong)

There are two dominant narratives about AI in software development right now, and I think both are fundamentally flawed:

### The AI Maximalists

These are the people saying AI will do everything. We're never going to need developers again. We won't need any white-collar workers. AI is coming for your job. Just give it user stories and it will give you an app. We're headed for a Skynet scenario where AI becomes fully autonomous.

This narrative is everywhere in tech media, on LinkedIn, in executive boardrooms. It's driving massive investment and creating enormous anxiety.

### The AI Skeptics

On the other side are people saying AI is evil, or that it's worthless, that it can't do anything useful, that it's all hype. They dismiss it entirely or focus only on the risks while ignoring the actual capabilities.

This narrative shows up in developer communities, among people threatened by change, and in certain policy circles.

**I think both positions are wrong.**

## AI Realism: What AI Actually Is

My view is that AI is a powerful tool for augmenting human intelligence, but it's not a replacement for human judgment. Here's what that means in practice:

### What AI Is Genuinely Good At

AI has a massive amount of information embedded in it. It can generate code quickly. It can help you explore solution spaces. It can be an incredible productivity multiplier when used properly.

When I'm building this website, AI helps me:
- Generate boilerplate code faster than I could write it
- Explore different architectural approaches quickly
- Find solutions to problems I haven't encountered before
- Translate between technologies I know and ones I'm learning

For developers with strong fundamentals, AI is like having a highly knowledgeable junior developer who works instantly but needs constant supervision.

### The Fundamental Problem

AI hallucinates constantly, and it doesn't know when it's wrong. It will very confidently tell you wrong things and keep telling you wrong things. Some models will double down when challenged. You can't work in an autonomous fashion with that. You need a human validating all the output.

This isn't a temporary limitation that will be fixed in the next model. It's fundamental to how these systems work. They're pattern-matching and probability engines, not reasoning engines. They don't understand what they're generating; they're predicting what tokens should come next based on massive training data.

### The Human Advantage

Developers aren't simply writing code. The best ones are taking business-level requirements and translating them into technical requirements while identifying issues and trade-offs. They're making nuanced judgments about:
- Architecture (how should this system be structured?)
- Performance (what are the actual bottlenecks?)
- Maintainability (will we regret this in six months?)
- Business value (is this solving the actual problem?)
- Trade-offs (what are we giving up to get this benefit?)

They're asking the questions that need to be asked before writing a single line of code.

AI struggles with this because it doesn't know what it doesn't know. It can't reliably identify when it's making a bad assumption or missing an important consideration. It can't have the conversation where you realize the client is asking for the wrong thing.

## The Real Transformation

What excites me about AI isn't that it will replace developers. It's that it will transform what development looks like.

### The Productivity Multiplier

AI-augmented developers with strong fundamentals, good communication skills, and sound judgment will be incredibly productive. They'll be able to:
- Move faster through routine tasks
- Explore more architectural options
- Deliver better solutions because they can iterate more quickly
- Focus their time on the hard problems that require human insight

I use AI heavily in building this website. It lets me work in technologies I haven't used professionally (Rust, Nuxt, Postgres optimization) and learn them much faster than I could through documentation alone. But I'm validating everything, making the architectural decisions, and catching the mistakes.

### The Rising Bar

The bar is rising. If you were a mid-level developer who was never going to progress beyond that, who turned out middling quality code without asking the big questions, then yes, AI can probably do your job better than you can.

But if you understand data structures, algorithms, architecture, and trade-offs—if you can communicate effectively and make sound judgments—then AI makes you more valuable, not less.

This is why I tell people entering the field now that they need to understand fundamentals. You can't just know enough to be productive on a web project anymore. You need to understand how things actually work from an architectural and algorithmic perspective.

If all you can do is what AI can do, you're in trouble. But if you can do what AI can't—understand context, make nuanced judgments, identify bad assumptions, communicate with stakeholders, think architecturally—then you're incredibly valuable.

## The Broader AI Landscape

Generative AI has opened the door to other AI technologies being more readily utilized: classification, image classification, bounding algorithms, anomaly detection. These things existed before, but they weren't being widely deployed. Now that generative AI is such a buzzword, you can sneak other useful AI technologies in the back door.

We work in manufacturing and oil and gas, and there's massive opportunity for using AI in:
- Quality assurance (detecting defects in production)
- Predictive maintenance (identifying equipment likely to fail)
- Process optimization (finding inefficiencies in complex systems)

But it's always AI plus human expertise, not AI replacing human expertise. The AI can identify patterns in massive datasets that humans would miss. But it takes human judgment to know which patterns matter, what to do about them, and how to implement changes without creating new problems.

## What This Means for Different Audiences

### For Companies

Don't believe the hype that AI will replace your entire development team. But do invest in AI augmentation for your developers. The productivity gains are real if you have people with strong fundamentals who can use these tools effectively.

Focus on:
- Training your team to use AI effectively
- Establishing processes for validating AI-generated code
- Identifying tasks where AI augmentation provides the biggest multiplier
- Maintaining standards for architecture and code quality

### For Developers

If you're early in your career, focus on fundamentals. Learn data structures, algorithms, systems design, architecture. Don't just learn enough to be productive—understand how things actually work.

Develop your communication skills. Being able to talk to non-technical stakeholders, understand business requirements, and explain technical trade-offs is increasingly important.

Learn to work WITH AI, not against it or ignoring it. Figure out where it helps you and where it gets in your way. Develop processes for using it effectively while maintaining quality.

### For People Entering the Field

The opportunity is still enormous, but the requirements are higher. You need:
- Strong fundamentals (can't be skipped)
- Good communication skills (always mattered, matters more now)
- Ability to work with AI tools (table stakes)
- Sound judgment about when to trust AI output and when to question it

The "code bootcamp graduate who grinds out CRUD apps" path is closing. The "developer who thinks architecturally and communicates well" path is opening wider.

## My Approach: Practicing What I Preach

This personal website is my laboratory for AI-augmented development. I'm using technologies I haven't used professionally:
- Rust with Actix for the backend
- Nuxt for the frontend
- Postgres with UUIDv7 for the database

I'm heavily using AI (Claude, specifically) to help me learn these technologies and build the site. But I'm also validating everything, making the architectural decisions, and learning deeply about how these systems work.

The result is that I'm learning faster than I would through documentation alone, but I'm also building genuine expertise. The AI accelerates the learning curve; it doesn't replace the learning.

This is the model I advocate for: AI as a powerful tool that makes skilled humans more productive, not as a replacement for human expertise.

## The Talks I'm Giving

I've been evangelizing AI realism to various industries: oil and gas, manufacturing, healthcare. The message is consistent:

**Don't believe the hype, but don't dismiss the technology.** AI won't replace your experts, but it can make them dramatically more effective if you implement it thoughtfully.

**Focus on augmentation, not automation.** The goal isn't to eliminate human judgment. It's to give humans better tools for making judgments.

**Invest in your people.** The companies that will win are the ones that upskill their workforce to use AI effectively, not the ones that try to replace their workforce with AI.

**Maintain standards.** AI can generate a lot of code quickly. If you don't have strong code review processes and architectural standards, you'll end up with a mess that's harder to maintain than what you had before.

## Where This Goes Next

Looking forward, I expect:

**Short term (1-3 years):**
- AI-augmented development becomes standard practice
- Productivity gains become obvious in organizations that implement it well
- The gap widens between developers who can use AI effectively and those who can't
- We see some high-profile failures from companies that tried to go "AI-first" without human oversight

**Medium term (3-7 years):**
- AI gets better at certain specialized tasks (code generation, refactoring, testing)
- But the fundamental limitation (can't reason, can't know what it doesn't know) remains
- Successful organizations have figured out the right division of labor between AI and humans
- The role of "developer" evolves but doesn't disappear

**Long term (7+ years):**
- Hard to predict, but I'm skeptical of "AI will do everything" scenarios
- More likely: AI becomes one tool among many that skilled professionals use
- The premium on human judgment, communication, and architectural thinking increases
- New specializations emerge around AI integration and validation

## The Bottom Line

AI is neither savior nor threat. It's a tool. A powerful one, but still a tool.

The future belongs to people who can use AI effectively while maintaining the human capabilities that AI can't replicate: judgment, creativity, communication, understanding context, asking the right questions.

If that's you, you have nothing to fear and much to gain. If you're trying to compete with AI on the things AI does well while ignoring the things only humans can do, you're going to struggle.

The question isn't "Will AI replace me?" The question is "How can I use AI to become dramatically better at the work only I can do?"

That's the question I'm trying to answer in my own work, and it's the question I'm helping organizations think through in theirs.

---

*Navigation: [← Back to Professional Path](/about/professional) | [Continue to Life Now →](/about/now)*

---

## Metadata Notes for Development

**Interactive Elements:**
- Link back to Professional section
- Links to any talks/presentations if available
- Expandable section with "Learn More" resources on AI

**Photo Placeholders:**
1. Hero: Developer working with AI tools or collaborative human-AI image
2. Presenting at conference or workshop
3. This website project workspace

**Sidebar Elements:**
- "Key Principles" box summarizing AI realism
- "For Different Audiences" quick links (Companies, Developers, Newcomers)
- Contact for speaking engagements if appropriate

**Design Notes:**
- Keep this accessible to non-technical readers
- Use concrete examples over abstract theory
- Balance optimism with realism
- Position as thought leadership content