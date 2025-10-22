<template>
  <AboutLayout>
    <h1>From AI to IA</h1>

    <img
      src="~/assets/images/working-with-ai.jpg"
      alt="Working with AI"
      class="w-full max-w-2xl mx-auto rounded-lg shadow-lg my-6"
    />

    <p>
      There are two camps when it comes to AI in software development, and I think
      they're both asking the wrong question. The evangelists promise AI will replace
      programmers tomorrow. The skeptics dismiss it as worthless hype. But both perspectives
      share a flawed premise: they're debating whether AI <em>will</em> replace humans when
      the real issue is that autonomous AI <em>can't</em> work the way they think it can.
    </p>

    <p>
      The answer isn't finding middle ground between these positions. The answer is reframing
      the entire conversation. We're not talking about AI (Artificial Intelligence) replacing
      humans. We're talking about <strong>IA (Intelligence Augmentation)</strong>: technology
      that amplifies human capabilities rather than attempting to replace human judgment.
    </p>

    <h3>A Note on Terminology</h3>

    <p>
      Throughout this piece, I'll use more precise language than the "AI" buzzword allows.
      When referring to the actual technology, I'll use <strong>generative models (GM)</strong>—systems
      that generate text, images, code, video, and other outputs based on patterns in training data.
      This includes large language models (LLMs) like ChatGPT and Claude, image generators like
      Midjourney and DALL-E, and code assistants like GitHub Copilot.
    </p>

    <p>
      I'll reserve "AI" for discussing the broader narrative and hype. The distinction matters:
      calling these systems "intelligence" accepts the premise that they think or reason. They don't.
      They're pattern-matching engines that generate plausible outputs.
    </p>

    <h2>The Two Dominant Narratives</h2>

    <p>
      Let me explain why both camps are operating from the wrong premise:
    </p>

    <h3>The AI Maximalists</h3>

    <p>
      These are the people saying AI will do everything. We're never going to need
      developers again. We won't need any white-collar workers. AI is coming for your
      job. Just give it user stories and it will give you an app. We're headed for a
      Skynet scenario where AI becomes fully autonomous.
    </p>

    <p>
      This narrative is everywhere in tech media, on LinkedIn, in executive boardrooms.
      It's driving massive investment and creating enormous anxiety.
    </p>

    <h3>The AI Skeptics</h3>

    <p>
      On the other side are people saying AI is evil, or that it's worthless, that it
      can't do anything useful, that it's all hype. They dismiss it entirely or focus
      only on the risks while ignoring the actual capabilities.
    </p>

    <p>
      This narrative shows up in developer communities, among people threatened by
      change, and in certain policy circles.
    </p>

    <p>
      <strong>Both positions accept the premise that AI could theoretically replace human
      expertise—they just disagree on the timeline and desirability.</strong> But that premise
      itself is flawed.
    </p>

    <h2>Intelligence Augmentation: What We're Actually Building</h2>

    <p>
      Intelligence Augmentation (IA) isn't just a rebranding of "AI used responsibly." It's
      a fundamentally different framework that recognizes these systems as tools for amplifying
      human capabilities, not substitutes for human judgment. Here's what that means in practice:
    </p>

    <h3>What Generative Models (GMs) Are Good At</h3>

    <p>
      GMs have a massive amount of information embedded in their training data.
      They can generate code quickly. They can help you explore solution spaces. They can be
      incredible productivity multipliers when used properly within an IA framework.
    </p>

    <p>When I'm building this website, GMs help me:</p>

    <ul class="list-disc list-inside mb-4 space-y-2">
      <li>Generate boilerplate code faster than I could write it</li>
      <li>Explore different architectural approaches quickly</li>
      <li>Find solutions to problems I haven't encountered before</li>
      <li>Translate between technologies I know and ones I'm learning</li>
    </ul>

    <p>
      For developers with strong fundamentals, working with GMs is like having
      a highly knowledgeable junior developer who works instantly but needs constant supervision.
    </p>

    <h3>The Fundamental Problem: Why Autonomous AI Can't Work</h3>

    <p>
      GMs hallucinate constantly, and they don't know when they're wrong.
      They will very confidently tell you incorrect things and keep doubling down when challenged.
      You can't work autonomously with systems that behave this way. You need humans validating
      all output.
    </p>

    <p>
      This isn't a temporary limitation that will be fixed in the next model.
      <SteampunkTooltip variant="steel">
        <template #content>
          Academic research has proven that hallucination is an inevitable, innate limitation of GMs.
          Even with larger training sets, structural hallucinations cannot be eliminated.
          <br><br>
          Sources:<br>
          • <a href="https://arxiv.org/abs/2401.11817" target="_blank" class="text-amber-700 underline">Hallucination is Inevitable (arXiv 2401.11817)</a><br>
          • <a href="https://arxiv.org/html/2409.05746v1" target="_blank" class="text-amber-700 underline">LLMs Will Always Hallucinate (arXiv 2024)</a>
        </template>
        It's fundamental to how these systems work.
      </SteampunkTooltip>
      They're pattern-matching and probability engines, not reasoning engines. They don't
      understand what they're generating; they're predicting what tokens should come next
      based on massive training data.
    </p>

    <p>
      Even more concerning:
      <SteampunkTooltip variant="steel">
        <template #content>
          Anthropic's research on Claude 3.5 showed that "the steps Claude used to solve problems
          were not what we expected—they're also not the steps that Claude claimed it took, providing
          clear evidence that large language models will give reasons for what they do that do not
          necessarily reflect what they actually did."
          <br><br>
          Source: <a href="https://www.anthropic.com/research/tracing-thoughts-language-model" target="_blank" class="text-amber-700 underline">Anthropic: Tracing Thoughts of a Language Model</a>
        </template>
        the companies building these systems admit they don't fully understand how they work.
      </SteampunkTooltip>
      Anthropic CEO Dario Amodei has acknowledged that "people are often surprised and alarmed to
      learn that we do not understand how our own AI creations work"—calling this unprecedented
      in the history of technology.
    </p>

    <p>
      <strong>This is why the "AI will replace humans" narrative is fundamentally flawed.</strong>
      You cannot safely deploy fully autonomous systems that hallucinate unpredictably, can't
      recognize their own errors, and whose decision-making processes are opaque even to their
      creators.
    </p>

    <h3>The Human Advantage</h3>

    <p>
      Developers aren't simply writing code. The best ones are taking business-level
      requirements and translating them into technical requirements while identifying
      issues and trade-offs. They're making nuanced judgments about:
    </p>

    <ul class="list-disc list-inside mb-4 space-y-2">
      <li>Architecture (how should this system be structured?)</li>
      <li>Performance (what are the actual bottlenecks?)</li>
      <li>Maintainability (will we regret this in six months?)</li>
      <li>Business value (is this solving the actual problem?)</li>
      <li>Trade-offs (what are we giving up to get this benefit?)</li>
    </ul>

    <p>
      They're asking the questions that need to be asked before writing a single line of
      code.
    </p>

    <p>
      GMs struggle with this because they don't know what they don't know. They can't
      reliably identify when they're making a bad assumption or missing an important
      consideration. They can't have the conversation where you realize the client is
      asking for the wrong thing.
    </p>

    <h2>The Real Transformation: Intelligence Augmentation in Practice</h2>

    <p>
      What excites me about IA isn't that technology will replace human expertise. It's that
      Intelligence Augmentation will transform what human experts can accomplish.
    </p>

    <h3>The Productivity Multiplier</h3>

    <p>
      Developers practicing IA—combining strong fundamentals, good communication skills, and
      sound judgment with AI tools—will be incredibly productive. They'll be able to:
    </p>

    <ul class="list-disc list-inside mb-4 space-y-2">
      <li>Move faster through routine tasks</li>
      <li>Explore more architectural options</li>
      <li>Deliver better solutions because they can iterate more quickly</li>
      <li>Focus their time on the hard problems that require human insight</li>
    </ul>

    <p>
      I practice IA extensively in building this website. Intelligence Augmentation lets me
      work in technologies I haven't used professionally (Rust, Nuxt, Postgres optimization)
      and learn them much faster than I could through documentation alone. But I'm validating
      everything, making the architectural decisions, and catching the mistakes. The GM
      accelerates my capabilities—it doesn't replace my judgment.
    </p>

    <h3>The Rising Bar</h3>

    <p>
      The bar is rising. If you were a mid-level developer who was never going to
      progress beyond that, who turned out middling quality code without asking the big
      questions, then yes, GMs can probably do your job better than you can.
    </p>

    <p>
      But if you understand data structures, algorithms, architecture, and trade-offs, if
      you can communicate effectively and make sound judgments, then GMs make you more
      valuable, not less.
    </p>

    <p>
      This is why I tell people entering the field now that they need to understand
      fundamentals. You can't just know enough to be productive on a web project
      anymore. You need to understand how things actually work from an architectural and
      algorithmic perspective.
    </p>

    <p>
      If all you can do is what GMs can do, you're in trouble. But if you can do what GMs
      can't (understand context, make nuanced judgments, identify bad assumptions,
      communicate with stakeholders, think architecturally), then you're incredibly
      valuable.
    </p>

    <h2>The Broader Landscape: GM and ML</h2>

    <p>
      Generative models (GMs) have opened the door to other machine learning (ML) technologies
      being more readily utilized: classification, image classification, bounding algorithms,
      anomaly detection. These things existed before, but they weren't being widely deployed.
      Now that generative AI is such a buzzword, you can sneak other useful ML technologies
      in the back door.
    </p>

    <p>
      We work in manufacturing and oil and gas, and there's massive opportunity for
      using ML in:
    </p>

    <ul class="list-disc list-inside mb-4 space-y-2">
      <li>Quality assurance (detecting defects in production)</li>
      <li>Predictive maintenance (identifying equipment likely to fail)</li>
      <li>Process optimization (finding inefficiencies in complex systems)</li>
    </ul>

    <p>
      But it's always Intelligence Augmentation—GMs and ML technologies amplifying human
      expertise, never replacing it. These systems can identify patterns in massive datasets
      that humans would miss. But it takes human judgment to know which patterns matter, what
      to do about them, and how to implement changes without creating new problems. This is IA
      in action: technology making human experts more capable.
    </p>

    <h2>What This Means for Different Audiences</h2>

    <h3>For Companies</h3>

    <p>
      Don't believe the hype that AI will replace your entire development team or expert
      workforce. But do invest in Intelligence Augmentation. The productivity gains are real
      when you have people with strong fundamentals who can practice IA effectively.
    </p>

    <p>Focus on:</p>

    <ul class="list-disc list-inside mb-4 space-y-2">
      <li>Training your team to practice IA—using GMs and ML tools while maintaining human judgment</li>
      <li>Establishing processes for validating GM-generated output</li>
      <li>Identifying tasks where IA provides the biggest productivity multiplier</li>
      <li>Maintaining standards for architecture, quality, and human oversight</li>
    </ul>

    <h3>For Developers and People Entering the Field</h3>

    <p>
      The opportunity is still enormous, but the requirements are higher. The bar is rising.
      You need:
    </p>

    <ul class="list-disc list-inside mb-4 space-y-2">
      <li>
        <strong>Strong fundamentals</strong> - Data structures, algorithms, systems design,
        architecture. You can't just know enough to be productive. You need to understand how
        things actually work.
      </li>
      <li>
        <strong>Communication skills</strong> - Being able to talk to non-technical stakeholders,
        understand business requirements, and explain technical trade-offs is increasingly important.
      </li>
      <li>
        <strong>IA proficiency</strong> - Learn to practice Intelligence Augmentation. Figure out
        where GMs amplify your capabilities and where they get in your way.
      </li>
      <li>
        <strong>Sound judgment</strong> - Knowing when to trust GM output and when to question it.
        Developing processes for leveraging IA while maintaining quality.
      </li>
    </ul>

    <p>
      The "code bootcamp graduate who grinds out CRUD apps" path is closing. The
      "developer who thinks architecturally and communicates well" path is opening
      wider.
    </p>

    <h2>My Approach: Practicing Intelligence Augmentation</h2>

    <p>
      This personal website is my laboratory for Intelligence Augmentation in practice. I'm using
      technologies I haven't used professionally:
    </p>

    <ul class="list-disc list-inside mb-4 space-y-2">
      <li>Rust with Actix for the backend</li>
      <li>Nuxt for the frontend</li>
      <li>Postgres with UUIDv7 for the database</li>
    </ul>

    <p>
      I'm practicing IA throughout this project, using Claude to help me learn these technologies
      and build the site. But I'm also validating everything, making the architectural decisions,
      and learning deeply about how these systems work.
    </p>

    <p>
      The result is that I'm learning faster than I would through documentation alone, but I'm
      also building genuine expertise. The GM accelerates the learning curve; it doesn't replace
      the learning. My capabilities are augmented, not substituted.
    </p>

    <p>
      This is Intelligence Augmentation: GMs as powerful tools that amplify skilled human
      capabilities, not as replacements for human expertise.
    </p>

    <h2>The Talks I'm Giving: Spreading the IA Message</h2>

    <p>
      I've been evangelizing Intelligence Augmentation to various industries: oil and gas,
      manufacturing, healthcare. The message is consistent:
    </p>

    <p>
      <strong>Don't believe the hype, but don't dismiss the technology.</strong> AI
      won't replace your experts, but Intelligence Augmentation can make them dramatically
      more effective if you implement it thoughtfully.
    </p>

    <p>
      <strong>Practice IA, not autonomous automation.</strong> The goal isn't to eliminate
      human judgment. It's to give humans more powerful tools that amplify their capabilities
      and enable better decision-making.
    </p>

    <p>
      <strong>Invest in your people.</strong> The companies that will win are the ones
      that upskill their workforce to practice IA effectively, not the ones that try to
      replace their workforce with autonomous AI.
    </p>

    <p>
      <strong>Maintain human oversight and standards.</strong> GMs can generate output quickly,
      but without strong validation processes, architectural standards, and human judgment,
      you'll end up with a mess that's harder to maintain than what you had before.
    </p>

    <h2>Where This Goes Next</h2>

    <p>Looking forward, I expect:</p>

    <p><strong>Short term (1-3 years):</strong></p>
    <ul class="list-disc list-inside mb-4 space-y-2">
      <li>Intelligence Augmentation becomes standard practice across industries</li>
      <li>Productivity gains become obvious in organizations that implement IA effectively</li>
      <li>
        The gap widens between professionals who practice IA effectively and those who resist
        or misunderstand it
      </li>
      <li>
        We see high-profile failures from companies that tried to deploy autonomous AI
        without adequate human oversight
      </li>
    </ul>

    <p><strong>Medium term (3-7 years):</strong></p>
    <ul class="list-disc list-inside mb-4 space-y-2">
      <li>
        GMs improve at certain specialized tasks (code generation, refactoring, testing),
        making IA even more powerful
      </li>
      <li>
        But the fundamental limitations (hallucination, inability to reason, can't know what
        they don't know) remain inherent to the architecture
      </li>
      <li>
        Successful organizations have mastered Intelligence Augmentation—they've figured out
        the optimal division of labor between GMs and human judgment
      </li>
      <li>Professional roles evolve to emphasize human strengths while leveraging IA</li>
    </ul>

    <p><strong>Long term (7+ years):</strong></p>
    <ul class="list-disc list-inside mb-4 space-y-2">
      <li>Hard to predict, but I'm skeptical of "AI will do everything" scenarios</li>
      <li>
        More likely: GMs become one tool among many that skilled professionals use
      </li>
      <li>
        The premium on human judgment, communication, and architectural thinking
        increases
      </li>
      <li>New specializations emerge around GM integration and validation</li>
    </ul>

    <h2>The Bottom Line: From AI to IA</h2>

    <p>
      Generative models are neither savior nor threat. They're tools—powerful ones that work
      best when augmenting human capabilities, not attempting to replace human judgment.
    </p>

    <p>
      The future belongs to people who practice Intelligence Augmentation effectively:
      those who leverage GMs while maintaining the human capabilities that GMs can't
      replicate—judgment, creativity, communication, understanding context, asking the
      right questions.
    </p>

    <p>
      If that's you, you have nothing to fear and much to gain. If you're trying to compete
      with GMs on the things GMs do well while ignoring the things only humans can do, or if
      you're waiting for AI to replace human expertise, you're going to struggle.
    </p>

    <p>
      The question isn't "Will AI replace me?" The question is "How can I practice Intelligence
      Augmentation to become dramatically better at the work only I can do?"
    </p>

    <p>
      That's the question I'm exploring in my own work, and it's the question I'm helping
      organizations answer in theirs. Not AI replacing humans—IA amplifying human potential.
    </p>

    <AccessPrompt />
  </AboutLayout>
</template>

<script setup>
import AboutLayout from "~/components/About/AboutLayout.vue";
import AccessPrompt from "~/components/About/AccessPrompt.vue";
import SteampunkTooltip from "~/components/Steampunk/SteampunkTooltip.vue";

useHead({
  title: "From AI to IA - Kenn Williamson",
  meta: [
    {
      name: "description",
      content:
        "Intelligence Augmentation (IA) over Artificial Intelligence (AI): Why human judgment remains essential and how IA amplifies human capabilities rather than replacing them.",
    },
  ],
});

// Social media sharing (same professional image as /about/professional)
useSocialShare({
  title: "From AI to IA: Intelligence Augmentation - Kenn Williamson",
  imageKey: "professional"
});
</script>
