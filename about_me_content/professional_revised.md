# Professional Path

*[Hero image: You at work, or at a whiteboard, or professional headshot]*

I was sitting in a cubicle designing compressor stations when I realized I'd spent ten years getting a degree for work that bored me to tears. Each career pivot since then has taught me something about finding work that actually engages me rather than work I thought I should do.

## From Engineering to Automation

After finally completing my mechanical engineering degree in 2010, I started working for Dr. Morton on his educational platform, Expert TA. It was a system for math, engineering, and physics education, and I was doing support work and helping with development.

Then I got what I thought was a "real" engineering job at Willbros, an oil and gas design company. I was designing compressor stations. It should have been exactly what I'd spent ten years working toward. But the work wasn't satisfying. It was fine, it paid well, but it didn't engage me the way I'd hoped.

When my mom got sick, I used that as an opportunity to leave and get more flexible work. I went back to working for Dr. Morton and also started contracting for Roy Farrell at HR Value. Roy's company created sophisticated spreadsheet automations for HR departments to manage their complex data.

This is where something clicked. I'd already developed a liking for Visual Basic for Applications (VBA) in Excel during college and at Willbros. With Roy, I started creating sophisticated automated HR spreadsheets. My development skills grew significantly. For about three to four years after my mom died in December 2013, I was in this hybrid world: part engineering support for Dr. Morton, part spreadsheet developer for Roy.

I was good at it. Very good at it. And I enjoyed it more than mechanical engineering work. But I didn't yet recognize it as a career pivot. I still thought of myself as an engineer who happened to be good with spreadsheets.

*[Photo: Early career era or engineering work]*

## The Crisis and the Pivot

A few years into this arrangement, everything fell apart. My business relationship with Roy ended badly. My mom passed away. I was going through the destructive period I described in the previous section. Professionally, I was adrift.

I tried to find another engineering job. I even applied to be a supervisor back at the wastewater plant, but I didn't get a callback. I was at a crossroads: I could try to force my way back into mechanical engineering, or I could lean into what I'd actually been enjoying, which was development work.

I'd been interested in web development since my earliest days at Expert TA. Their platform was web-based, and their lead developer, Daniel Vail, was brilliant. I saw him as a kind of guru and wanted to learn from him. I asked Dr. Morton if I could help Daniel with web development work, but he shut it down: "That's a completely different skill set. Let the web guys do the web stuff and us engineering guys do engineering stuff."

With my other options closing, I decided to pursue that interest seriously. I enrolled in a 16-week boot camp at Coding Dojo in 2017.

## Coding Dojo: All In

The boot camp was intense. It was one of those programs where you get out what you put in, and I put in everything. I worked harder than I'd worked at anything since military school, but this time it was because I wanted to, not because I was forced to.

I did very well and developed a strong capstone project. Near the end of the program, our class took a tour of a local software company's facility. The CEO (then President) and Director of Operations were there, and the CEO asked our group some technical questions. I was the only one who could answer them.

That company was SEQTEK, I joined in 2018 and I've been there ever since.

*[Photo: Coding Dojo graduation or early SEQTEK days]*

## The SEQTEK Journey: Years of Growth

I interviewed at SEQTEK and got hired in February 2018. Interestingly, my very first project wasn't even web development. It was an embedded C/C++ project for a large multinational conglomerate's pipeline measurement division. We successfully upgraded their field application from an old single-threaded Windows system to a modern multi-threaded Linux system where the threads communicated over TCP.

It was completely different from my boot camp training, but I learned quickly. That's been the pattern throughout my career: throw me at a problem, give me time to research how others have solved it, and I'll figure it out.

### Project Progression

My career at SEQTEK has been defined by a series of successful projects:

**Pipeline Measurement System** (First project): Embedded C/C++ work upgrading legacy field applications to modern architecture. This was completely outside my boot camp training—I had to learn systems programming from scratch. Without AI available at the time, I researched C/C++ through articles and blogs, traced execution paths manually through the codebase, and learned through trial and error. My tech lead had the overall approach; I filled in details and problem-solved. One breakthrough was getting the code to compile locally, transferring it to the device, and seeing the output changes we were looking for. The challenge was decoupling a single-threaded application from the system boot process and creating a manager that would spawn separate threads for modular applications, all communicating over TCP. It taught me systems programming fundamentals that my web-focused boot camp never covered.

**MudMetrix** (SaaS transformation): We rebuilt an old desktop-based tracking software called "Mud Pro" into a modern SaaS product. Working with the product owner, we wireframed entirely new screens with the client and developed all new calculations using industry best practices rather than trying to reverse-engineer the old system. The biggest technical challenge was implementing offline functionality using Service Workers—brand new technology at the time. We had to store requests and responses in IndexedDB, generate IDs in the UI to maintain foreign key relationships, and handle sync ordering to prevent duplicates. Because the offline feature wasn't used frequently and development was moving fast, bugs would go undetected for months, causing data to get stuck on remote devices. The product owner and I created an offline testing regression suite that QA ran for every release, ensuring new functionality didn't break existing offline capabilities. This was all trial-and-error learning with brand-new technology that had few established patterns to follow.

**Credit Industry Platform**: Major rewrite and integration for a company in the credit space. The project involved establishing API connections with credit bureaus, implementing custom-built decision matrices, and creating a highly customizable form builder. I approached the unfamiliar domain by compartmentalizing—I didn't need to understand all of credit to connect APIs, just the data mapping: what the bureau needed to identify a person, what we'd get back, and which specific pieces we needed. This was my first exposure to high-compliance industries with PII considerations, where certain data could be stored or logged and other data couldn't. Instead of playing whack-a-mole with specific issues, we focused on fundamentals: What's the overall goal of this technology? How is what we're doing serving or hindering that goal? We triaged problems into easy fixes and hard fixes, knocked out all the easy ones, then tackled the harder ones like their brittle DocuSign integration. They claimed they only changed documents once a year, but we'd already done it three times since I started—reality was bursts of 2-3 versions before going untouched for a year. We re-engineered it to be robust: change the document as many times as you want, as long as you generate it with the right metadata DocuSign needs. This pattern of fixing fundamentals rather than symptoms gave their clients confidence and enabled the company to be acquired.

**Other Successful Deliveries**: Pipeline inspection software for a corrosion specialist, healthcare space applications, and numerous other projects where we delivered on time and on spec.

The consistent thread: I show up, I figure out what needs to be done, I deliver. Sometimes it's in technologies I've never used before. Sometimes it's in domains I don't fully understand initially. But I have a process.

My general methodology when facing unfamiliar territory: I don't wait until I fully understand everything before starting. Problem-solving has always been my core strength, and I walk in assuming I can deliver. There's a political aspect to consulting—clients have often been burned by contractors who made big promises, planted stakes, and left a mess. You have to build trust quickly by delivering value immediately.

Because of this, I can't spend time ramping up completely. The first question I ask is: "What's the biggest pain point we can solve right away?" So often, these problems involve circular dependencies, and you just have to start somewhere. You pick a point and work outward. When you come back around, you may need to rework some assumptions, but the whole problem space is too big to process at once. Find the lowest-hanging fruit where you can add value immediately, and start working.

I learn about the domain along the way. I never feel the need to become the expert—I rely on subject matter experts to guide me. My expertise is in problem-solving and architecting good software solutions. I apply my skills to their domain using their knowledge. I have a good memory and learn quickly, so I become decently familiar with the domain, but it isn't necessary to get started. You just get in there and start working.

### Title Progression

My track record of successful projects led to steady advancement:
- Started as a **Mid-Level Engineer**
- Promoted to **Senior Engineer**
- Promoted to **Technical Lead**
- Current title: **Enterprise Architect**

There's talk of another promotion related to AI work, which would be fitting given where I'm spending most of my energy these days.

*[Photo: Team photo or presenting at a conference]*

## The Mentors Who Shaped My Professional Growth

I've been incredibly fortunate to have mentors who invested in my development:

**Hank Haines (CEO)**: Hank has been a huge mentor for me, both in work and in life. He's taught me about leadership, about integrity, about doing business the right way.

**Greg Wonderly (First Technical Lead)**: Greg was the team lead on my first project. He was an incredibly influential technical mentor who taught me how to approach unfamiliar problems systematically.

**Brent Fields (President)**: When I was hired, Brent was the Director of Operations. He's now the President, and he's also my best friend. Brent has been instrumental in teaching me how to leverage my social and communication skills professionally. I used to have this persona of "technical excellence with minimal communication," but Brent showed me how to use my communication abilities to become a better leader. That's been crucial in my current role where I'm overseeing projects and mentoring other developers.

I've also worked with amazing product owners like Sam Haines and Andrew Lee, who constantly challenge me to improve my communication and to think about problems from a business perspective rather than just a technical one.

## Work-Life Integration

I don't believe in work-life balance as it's usually described. For me, work is part of life, not separate from it.

My boss is my best friend. We go to the same church. I'm friends with all the developers I work with. We hang out and do fun stuff together. We celebrate milestones in each other's lives. SEQTEK is a family-oriented company that genuinely cares about people.

This might not work in a giant corporation where you can't trust anyone and everyone's out for themselves. But for us, it works. We hold people to 40 hours a week unless something's going wrong or there's a big deadline. We manage client expectations to maintain reasonable pace. Everybody has a rich home life.

For me, the integration is complete. I build relationships with everyone I work with because I legitimately care about them. They're important to me as people, not simply as coworkers. That's how I operate.

## Current Focus and Future Direction

Right now, I'm working on several things simultaneously:

**Client projects**: I'm still an individual contributor on some projects, doing actual coding. I'm also serving as an architect on others, doing design work and aligning technical solutions with business strategy.

**AI evangelism**: I'm giving talks to various industries (oil and gas, manufacturing, healthcare) about AI realism and how to actually leverage these technologies effectively. (See my full thoughts on AI in the AI & The Future of Work section.)

**Mentoring**: I'm working with junior and mid-level developers, helping them understand not just how to write code but how to think about problems architecturally.

**This personal website**: It's a passion project where I'm using technologies I haven't used professionally (Rust with Actix on the backend, Nuxt on the frontend, Postgres for the database) to stay sharp and explore new approaches.

Looking forward five to ten years, I'd love to continue growing with SEQTEK. Sometimes I feel like a big fish in a small pond, just because I've succeeded at everything they've asked me to do. But we're working on expanding the company and creating more room for growth. I'm slowly transitioning into leadership roles.

I don't think I'll ever be heavily into people management in the HR sense. That's not my gifting. But technical leadership, helping people grow technically, making architectural decisions, evangelizing good practices... that's where I want to be.

Ultimately, my professional work is part of my kingdom work. I want to use my platform to be a witness, to love people well, to help organizations succeed in ways that benefit their employees and customers. That's what success looks like to me.

Beyond the work itself, there's the daily reality of trying to balance career with being present for three kids who need me.

---

*Navigation: [← Back to Theology & Practice](/about/theology) | [Continue to AI & The Future of Work →](/about/ai)*

---

## Metadata Notes for Development

**Interactive Elements:**
- Tooltip on "VBA" → Visual Basic for Applications, brief explanation
- Tooltip on "embedded C/C++" → explanation for non-technical readers
- Tooltip on "SaaS" → Software as a Service definition
- Link to SEQTEK website
- Links to talks/presentations if available
- Link to AI section for full philosophy

**Photo Placeholders:**
1. Hero: Professional photo, at whiteboard, or working
2. Early career era or engineering work
3. Coding Dojo graduation or early SEQTEK days
4. Team photo or presenting at conference
5. Current work setup or this personal website project

**Sidebar Elements:**
- Timeline: 2010 Engineering degree → 2017 Coding Dojo → Feb 2018-present SEQTEK progression
- "Tech Stack" box showing current technologies you work with
- Career advice highlights
- Link to AI section

**Design Notes:**
- Balance technical content with accessibility
- Show progression and pattern of success
- Emphasize mentorship and relationships
- Keep focus on people over technology
- AI content now lives in separate section