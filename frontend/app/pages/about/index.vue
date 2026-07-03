<template>
  <AboutLayout>
    <h1>About Kenn Williamson</h1>

    <!-- Professional headshot -->
    <img
      src="~/assets/images/headshot.jpg"
      alt="Kenn Williamson professional headshot"
      class="w-full max-w-md mx-auto rounded-lg shadow-lg my-6"
    />

    <h2>Present Tense</h2>

    <p>
      I'm a single dad to three remarkable kids: Rory ({{ roryAge }}), Charlie
      ({{ charlieAge }}), and Teddy ({{ teddyAge }}). By day, I'm an Enterprise
      Architect at SEQTEK, building software systems and helping companies cut through AI
      hype to figure out what actually works. By calling, I'm a disciple of Christ, still
      figuring out what it means to walk in love.
    </p>

    <p>
      I live in Jenks, Oklahoma, where I'm juggling work deadlines, dance recitals,
      autism therapy appointments, homeschool co-op volunteering, and an ambitious
      personal website project that lets me geek out on Rust and modern web
      architecture. I like to write music, blog posts, and code, play guitar reasonably
      well, and I'm a huge nerd for anime and way too many fantasy novels. I'm learning
      to hike mountains despite a fear of heights, trying to stay patient when I want to
      be critical, and working on living the grace I preach.
    </p>

    <p>
      This is my story. It's not a resume and it's not a highlight reel. It's the real
      arc: gifted kid who couldn't function in school, lost years after my mother died,
      transformation through faith I didn't expect, and now trying to be a good father,
      a faithful disciple, and someone who actually helps people instead of thinking
      about it.
    </p>

    <hr class="my-8 border-primary-300" />

    <h2>The Arc</h2>

    <p>
      I was born in Tulsa in 1982 to a family that moved from lower-middle class to
      upper-middle class over the course of my childhood. My dad was a lawyer and state
      legislator; my mom was a teacher. I had four grandparents who were all formative
      influences. Engineers and entrepreneurs, both sets teaching me different things
      about work, money, and life.
    </p>

    <p>
      On paper, I should have crushed it. High IQ, gifted programs, math competitions
      where I placed fourth in the state. In reality, I was the kid who could ace tests
      without studying but wouldn't turn in homework. The gap between my potential and
      my performance was a constant source of frustration for everyone, especially me.
      My parents divorced when I was ten. I got bullied. I changed schools five times.
      My senior year, my parents sent me to military school as a last-ditch
      intervention, and suddenly I got straight A's because I had no choice.
    </p>

    <p>
      College was supposed to be the fresh start. Instead, it was a ten-year odyssey of
      false starts, major changes, and dropping out multiple times. I finally graduated
      with a mechanical engineering degree at 28, but not before working at a wastewater
      treatment plant and many other temporary jobs.
    </p>

    <p>
      Then my mom got cancer. Watching her die while I was still figuring out my life
      sent me into a spiral I barely survived. I won't detail the specifics of that
      period. It involved substance abuse, self-destruction, and hurting people I cared
      about. What matters is that I hit bottom hard enough to realize I couldn't save
      myself.
    </p>

    <p>
      I met my co-parent during that broken season, and we had three kids together:
      Rory, Charlie, and Teddy. They are the best things that have ever happened to me,
      full stop. The relationship didn't last; we've been separated for over
      {{ separationDuration }} years and split custody, with me having marginally more
      time. Teddy has
      <SteampunkTooltip :clickable="true" placement="top">
        <template #default>
          <span>level three autism</span>
        </template>
        <template #content>
          <div>
            <p class="mb-2">
              Level 3 autism is classified as "requiring very substantial support" - the
              most severe classification within the autism spectrum disorder framework.
            </p>
            <p class="mb-2">
              <strong>Communication:</strong> Severe deficits in verbal and nonverbal
              communication, with minimal initiation of social interactions. Many
              children are nonverbal or use only a few words.
            </p>
            <p class="mb-3">
              <strong>Support needs:</strong> Marked inflexibility of behavior, extreme
              difficulty coping with changes to routine. May use AAC devices, PECS, or
              sign language to communicate.
            </p>
            <a
              href="https://www.autismspeaks.org/levels-of-autism"
              target="_blank"
              rel="noopener noreferrer"
              class="text-primary-800 hover:text-primary-900 underline font-semibold"
            >
              Learn more at Autism Speaks →
            </a>
          </div>
        </template> </SteampunkTooltip
      >, which is its own journey.
    </p>

    <p>
      The transformation didn't happen all at once. I found my way back to faith, but
      not the Midwestern Protestantism I grew up with. Discovering Jonathan
      Pageau's symbolic way of reading Scripture led me to
      Orthodox Christianity and the Church Fathers. (More on that journey, and the
      theology I've landed on, in
      <NuxtLink to="/about/faith" class="text-primary-700 hover:text-primary-900 underline"
        >Finding Faith</NuxtLink
      >
      and
      <NuxtLink
        to="/about/theology"
        class="text-primary-700 hover:text-primary-900 underline"
        >Theology &amp; Practice</NuxtLink
      >
      below.)
    </p>

    <p>
      Professionally, I went back to school, finished my engineering degree, and then,
      after my engineering work proved unsatisfying, pivoted into software development
      through a coding boot camp in 2017. I've been at SEQTEK for almost seven and a
      half years now, moving from mid-level developer to Enterprise Architect. I've led
      successful projects, helped companies get acquired, and found my niche
      evangelizing Intelligence Augmentation (IA), helping companies understand that
      the goal is amplifying human capabilities, not replacing human judgment.
    </p>

    <p>
      Now I'm here: single dad, software architect, amateur theologian, trying to raise
      three kids to walk in love while figuring out what that means myself. I'm critical
      of myself in ways I'm not critical of others. I struggle with patience, with
      self-regulation, with the fear that I'm still getting it all wrong. But I'm also
      building a life that has purpose beyond my own comfort, not just software.
    </p>

    <hr class="my-8 border-primary-300" />

    <h2>What You'll Find Here</h2>

    <p>
      This site is organized around the major chapters of my story. You can read
      straight through or jump to what interests you:
    </p>

    <ul class="space-y-3 my-6">
      <li>
        <NuxtLink
          to="/about/origins"
          class="font-semibold text-primary-700 hover:text-primary-900"
        >
          Origins
        </NuxtLink>: Family background, the grandparents who shaped me, education struggles, and
        the military school year that changed everything (for better and worse)
      </li>
      <li v-if="hasTrustedContactAccess">
        <NuxtLink
          to="/about/wilderness"
          class="font-semibold text-primary-700 hover:text-primary-900"
        >
          The Wilderness
        </NuxtLink>: College failures, my mother's death, the lost years, becoming a father while
        barely functional
      </li>
      <li>
        <NuxtLink
          to="/about/faith"
          class="font-semibold text-primary-700 hover:text-primary-900"
        >
          Finding Faith
        </NuxtLink>: From childhood religion through atheism to discovering the Church Fathers, and
        the journey back to faith
      </li>
      <li>
        <NuxtLink
          to="/about/theology"
          class="font-semibold text-primary-700 hover:text-primary-900"
        >
          Theology & Practice
        </NuxtLink>: Mimetic theory, the non-sacrificial reading of the Gospel, and what Christian
        Voluntarism actually means
      </li>
      <li>
        <NuxtLink
          to="/about/professional"
          class="font-semibold text-primary-700 hover:text-primary-900"
        >
          Professional Path
        </NuxtLink>: Engineering to software, the SEQTEK story, and my problem-solving approach
      </li>
      <li>
        <NuxtLink
          to="/about/ai"
          class="font-semibold text-primary-700 hover:text-primary-900"
        >
          From AI to IA
        </NuxtLink>: Why both AI hype and skepticism miss the point. Intelligence Augmentation amplifies human capabilities rather than replacing them
      </li>
      <li v-if="hasTrustedContactAccess">
        <NuxtLink
          to="/about/now"
          class="font-semibold text-primary-700 hover:text-primary-900"
        >
          Life Now
        </NuxtLink>: Single dad reality, three kids with very different needs, church community,
        hobbies, and yes, the dating question
      </li>
      <li>
        <NuxtLink
          to="/about/vision"
          class="font-semibold text-primary-700 hover:text-primary-900"
        >
          Philosophy & Vision
        </NuxtLink>: Core values, political-economic views, legacy hopes, and advice I wish I'd
        received earlier
      </li>
    </ul>

    <hr class="my-8 border-primary-300" />

    <h2>A Note on Tone</h2>

    <p>
      I'm usually pretty reserved about myself until someone asks. Then I'm open, maybe
      too open. This is the forum for that openness. You'll find vulnerability here, but
      not oversharing. I'm honest about my struggles without dwelling on them, and
      self-critical without losing sight of how far I've come. Throughout it all, I'm
      trying to point toward something bigger than myself: toward God, toward love,
      toward truth.
    </p>

    <p>Welcome to the story. It's still being written.</p>

    <AccessPrompt />
  </AboutLayout>
</template>

<script setup>
import AboutLayout from "~/components/About/AboutLayout.vue";
import SteampunkTooltip from "~/components/Steampunk/SteampunkTooltip.vue";
import AccessPrompt from "~/components/About/AccessPrompt.vue";
import { ageFromYearMonth, halfYearsSince, formatHalfYears } from "~/utils/age";
import { BIRTHDAYS } from "~/constants/birthdays";
import { MILESTONES } from "~/constants/milestones";

const { hasTrustedContactAccess } = useUserRoles()

// Ages are computed from birth year+month at render time (birth day omitted for
// privacy). useAboutNow() provides a shared, SSR-safe reference instant.
const now = useAboutNow();
const roryAge = ageFromYearMonth(BIRTHDAYS.rory, now);
const charlieAge = ageFromYearMonth(BIRTHDAYS.charlie, now);
const teddyAge = ageFromYearMonth(BIRTHDAYS.teddy, now);
const separationDuration = formatHalfYears(halfYearsSince(MILESTONES.separation, now));

useHead({
  title: "About Kenn Williamson",
  meta: [
    {
      name: "description",
      content:
        "The story of Kenn Williamson: single dad, software architect, disciple of Christ, exploring faith, family, and the intersection of technology and theology.",
    },
  ],
});

// Social media sharing
useSocialShare({
  title: "About Kenn Williamson",
  imageKey: "personal"
});
</script>
