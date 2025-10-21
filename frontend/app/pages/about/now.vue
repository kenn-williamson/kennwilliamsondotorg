<template>
  <AboutLayout>
    <h1>Life Now</h1>

    <div class="flex flex-col md:flex-row gap-6 my-6 max-w-4xl mx-auto">
      <img
        src="~/assets/images/family-candid-1.jpg"
        alt="Family selfie at Christmas light show"
        class="w-full md:w-1/2 rounded-lg shadow-lg"
      />
      <img
        src="~/assets/images/family-park-candid.jpg"
        alt="Family candid at the park"
        class="w-full md:w-1/2 rounded-lg shadow-lg"
      />
    </div>

    <p>
      This is where I am today: single dad to three remarkable kids, Enterprise
      Architect by day, disciple by calling, trying to figure out what it means to walk
      in love while managing the chaos of everyday life.
    </p>

    <h2>The Kids: Three Very Different People</h2>

    <p>
      I have Rory (10), Charlie (9), and Teddy (almost 5). They're with me most of the
      time, about four and a half days a week. Their mom and I split custody, with me
      having marginally more time. We maintain a healthy co-parenting relationship,
      which I'm grateful for.
    </p>

    <p>
      Each of my kids is completely unique, and I see different parts of myself and
      their family in each of them.
    </p>

    <h3>Aurora Maeve (Rory)</h3>

    <p>
      Rory is my oldest at 10, and she's like the girl version of me in so many ways. I
      see myself in her constantly. The interesting thing is that I'm raising her
      totally differently than my mom raised me. My mom pushed me to excel constantly,
      which I hated growing up but which, looking back, helped me achieve a lot. With
      Rory, I'm taking the opposite approach: letting her grow at her own pace,
      providing a good example rather than didactic instruction, not pushing her to
      perform.
    </p>

    <p>
      I don't know if this is the right approach. It's an experiment. But I want her to
      develop without the constant pressure I felt, while still hopefully giving her the
      tools she needs to succeed.
    </p>

    <h3>Charlotte Ann (Charlie)</h3>

    <p>
      Charlie is 9 and is so much like my mom. She's an amazing social butterfly, loves
      everyone, is incredibly kind. She can also have a sharp wit when she wants to.
      Watching her navigate social situations with an ease I never had is both beautiful
      and bittersweet. She reminds me so much of my mom, and I'm grateful for that.
    </p>

    <h3>Theodore Lysander (Teddy)</h3>

    <p>
      Teddy is almost 5 and has
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
              class="text-amber-800 hover:text-amber-900 underline font-semibold"
            >
              Learn more at Autism Speaks →
            </a>
          </div>
        </template> </SteampunkTooltip
      >. He can't talk yet, which is obviously a significant challenge. I see so much of
      myself in him, but it's hard to get to know him because he's difficult to
      understand. Dealing with his disability has been a challenge for both of us.
    </p>

    <p>
      We're doing therapy, working on equipping him with skills, hoping he'll be able to
      communicate more as he gets older. It's a journey, and honestly, I don't know
      where it's going to lead. But he's my son, and we're going to figure it out
      together.
    </p>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 my-8">
      <div class="text-center">
        <img
          src="~/assets/images/rory-portrait.jpg"
          alt="Rory"
          class="w-full rounded-lg shadow-lg mb-2"
        />
        <p class="text-sm font-semibold text-amber-800">Rory (10)</p>
      </div>
      <div class="text-center">
        <img
          src="~/assets/images/charlie-portrait.jpg"
          alt="Charlie"
          class="w-full rounded-lg shadow-lg mb-2"
        />
        <p class="text-sm font-semibold text-amber-800">Charlie (9)</p>
      </div>
      <div class="text-center">
        <img
          src="~/assets/images/teddyfinger.jpg"
          alt="Teddy"
          class="w-full rounded-lg shadow-lg mb-2"
        />
        <p class="text-sm font-semibold text-amber-800">Teddy (almost 5)</p>
      </div>
    </div>

    <h2>Daily Life: The Rhythms and Chaos</h2>

    <p>
      Every weekend is with my kids, and I love hanging out with them. The best weekends
      are when we have many activities and people to hang out with. I'm not the "sit
      down and play board games" dad. When we're home by ourselves, I tend to work on my
      website or other projects unless the kids need me. So having friends around and
      things for us to do together is ideal.
    </p>

    <p>
      I'm not one of those parents who super-programs the weekend with constant
      activities, though. We're not doing baseball tournaments all weekend. I want good
      people for them to hang out with and things to keep them engaged. Balance.
    </p>

    <h3>The Single Dad Reality</h3>

    <p>
      Being a single dad is hard in ways I didn't anticipate. The logistics alone are
      challenging: getting three kids to school, managing Teddy's therapy appointments,
      coordinating with their mom, handling the house, working full time. But the harder
      part is the isolation. Most of my friends are married with kids, and their lives
      are structured around their nuclear families. I'm not integrated into that in the
      same way.
    </p>

    <p>
      I'm also terrible at reaching out to people. My pattern is to assume I'm a bother,
      fear rejection, and stay by myself. The people who stick around in my life are the
      ones willing to reach out to me and who understand that my default is isolation.
      I'm working on this, but it's slow progress.
    </p>

    <h2>Church and Community</h2>

    <p>
      I'm at Church on the Move, which has been a good community for me. I'm involved in
      Bible study groups and small group ministries. I feel like I'm making some impact
      in encouraging and teaching newer believers, which is fulfilling.
    </p>

    <p>
      I'm also involved in a homeschool co-op that my friend Mark's wife helped me get
      connected with. It's an Aspire group, and I'm volunteering there, which helps my
      kids get some of the educational experiences and social connections I think are
      valuable. The homeschooling community has been welcoming, and I'm trying to get
      more embedded in that world.
    </p>

    <div class="photo-placeholder">Photo: Church or community gathering</div>

    <h2>Hobbies and Interests: The Nerdy Life</h2>

    <p>
      I've been a big nerd my whole life, and that hasn't changed. What has changed is
      what's practical with three kids.
    </p>

    <h3>Former Gamer</h3>

    <p>
      I used to play many video games. Unfortunately, modern video games are not made
      for dads whose children need to interrupt them constantly. Everything is real-time
      now with no pause function. I couldn't make the "gamer dad" thing work without a
      partner helping manage the kids, which my friends who game have.
    </p>

    <p>
      So I'm done with video games, much to the chagrin of my best friend from childhood
      who's a big gamer, and the game community I was involved with online. They're
      pretty annoyed with me, but that's the reality of being a dad.
    </p>

    <h3>Anime and Manga</h3>

    <p>
      I enjoy anime. I used to read manga frequently, but I do so much reading for work
      and theology that I don't have bandwidth for it anymore. I usually wait for the
      anime adaptation. I've even come around to dubbed anime in my old age, though I
      used to prefer subtitles.
    </p>

    <p>
      Some favorites: <em>Frieren: Beyond Journey's End</em>,
      <em>Legend of the Galactic Heroes</em>, <em>Cowboy Bebop</em>,
      <em>Berserk</em> (manga is better), <em>Vinland Saga</em>, <em>One Piece</em>,
      <em>Demon Slayer</em>. There are many more. Anime has been a constant throughout
      my life.
    </p>

    <h3>Reading</h3>

    <p>I'm a voracious reader. Right now I'm reading:</p>

    <ul class="list-disc list-inside mb-4 space-y-2">
      <li>The Church Fathers (Gregory of Nyssa, Maximus, Athanasius, Ephrem)</li>
      <li>René Girard (currently working through <em>Violence and the Sacred</em>)</li>
      <li>The Bible (using daily reading plans, though not always consistently)</li>
    </ul>

    <p>
      I used to read many sci-fi and fantasy novels. I've probably read over a thousand
      of them in my lifetime. <em>The Wheel of Time</em> (read it cover to cover maybe
      five or six times), <em>The Death Gate Cycle</em>, Raymond E. Feist's
      <em>Midkemia</em> series, Terry Pratchett's <em>Discworld</em>, Jim Butcher's
      <em>Dresden Files</em>. I loved literature and poetry too: Alexandre Dumas,
      Shakespeare, Oscar Wilde, Philip Larkin, the classics.
    </p>

    <p>
      But these days, most of my reading energy goes to theology and philosophy. My
      ex-girlfriend Kathryn got me into literature and poetry that broadened my horizons
      from pure sci-fi/fantasy, and I'm grateful for that.
    </p>

    <h3>Music: Guitar and Songwriting</h3>

    <p>
      I play guitar and write songs. My recent pattern is: I write the lyrics and the
      overall vision for the music, use a generative model to create a version that gets close to what
      I'm hearing in my head, then practice it on guitar and perform it myself. So it's
      my lyrics and vision, music written by a generative model, performed by me.
    </p>

    <p>
      I've been drawn into the website project recently, so music has taken a back seat,
      but it's something I will continue to do as I get more free time.
    </p>

    <h3>Fitness: The New Leaf</h3>

    <p>
      After military school, I didn't exercise at all except by accident. But recently
      I've turned a new leaf on fitness. I started hiking and have been enjoying it. I
      even hiked up and climbed to the top of a mountain recently, which was an
      interesting three-day experience and a genuine challenge given my fear of heights.
    </p>

    <p>
      I've been keeping up with training for hiking: stairs training, walking with a
      pack, staying in hiking shape. My friend found a good deal on a home gym (like a
      Bowflex), and I've been thinking about getting into some weightlifting to add
      strength training to my cardio work.
    </p>

    <p>
      I used to be a big swimmer. I was a lifeguard and swim instructor for five years
      and enjoyed it. But I haven't done much swimming recently because, as a single dad
      trying to practice purity, the pool is kind of a dangerous place for me.
    </p>

    <img
      src="~/assets/images/on_the_mountain.jpg"
      alt="On the mountain - hiking adventure"
      class="w-full max-w-2xl mx-auto rounded-lg shadow-lg my-6"
    />

    <h2>The Website Project</h2>

    <img
      src="~/assets/images/steampunk-doc-holiday.jpg"
      alt="Steampunk aesthetic - the theme of this website"
      class="w-full max-w-2xl mx-auto rounded-lg shadow-lg my-6"
    />

    <p>
      This personal website is a big passion project right now. I'm using it to learn
      technologies I haven't used professionally: Rust with Actix for the backend, Nuxt
      for the frontend, Postgres for the database with UUIDv7.
    </p>

    <p>
      I love that I can work on this and pause whenever the kids need me. It's
      intellectually engaging, lets me stay sharp on emerging technologies, and gives me
      something to show for the time I spend on it. Plus, I'm using generative models heavily in the
      development process, which lets me practice what I preach about Intelligence Augmentation.
    </p>

    <p>
      I've learned a ton working on this project, especially about Postgres database
      design and optimization, which is different from the SQL Server work I usually do
      professionally.
    </p>

    <h2>The Dating Question</h2>

    <p>
      I'm open about this: I'd love to find the right person to share life with. I'm a
      romantic at heart. I'm the guy who handmade a Halloween costume of a juice box with
      Betty Boop on it (we called it "Boop Juice"), who's crafted throw pillows and
      jewelry boxes, who stops at the store to pick up something just because I know
      she'd like it. I'm considerate. I think about how my actions affect my partner. I
      keep her on my mind: calling before making plans with friends, remembering the
      little things, doing sweet things for birthdays and anniversaries.
    </p>

    <p>
      Physical affection is important to me. I imagine cuddling on the couch reading or
      watching anime together, waking up before she does just to watch her sleep for a
      moment, cooking dinner side by side, having someone there when I get home. I want
      the everyday partnership: cleaning up the house together, working through
      frustrations with good communication and making up, having my bathroom cluttered
      with her makeup and curlers, downsizing my sprawling closet to one tiny rack so the
      rest can be hers. I want her to decorate the house because my walls are bare and I
      have no desire to pick out décor, but I'd love to live in a home that feels warm
      and beautiful because she made it that way.
    </p>

    <p>
      I do better with organic connection than dating apps. All of my previous serious
      relationships developed naturally: friends, friends of friends, coworkers, someone
      at the pool where I was a lifeguard. Dating apps require you to sell yourself
      immediately based on photos and a brief bio, and that's not my strength. I'm better
      in long-form conversation where depth and care for people can actually come
      through. Coffee at a local shop, walking through Philbrook Museum, dinner and a
      stroll along the Riverwalk. These are where I connect with people.
    </p>

    <p>
      Here's what I'm looking for: someone who's Kingdom-oriented, where faith is central
      rather than peripheral. Someone bubbly, vivacious, outgoing, the kind of person
      who's a natural connector, who can draw me out of my introverted default without
      getting frustrated when there's resistance. I need loyalty, sincerity, earnestness,
      someone hardworking who embodies traditional family values rather than focusing on
      climbing the corporate ladder. I want to build something together.
    </p>

    <p>
      I'm attracted to women who take care of themselves and carry themselves with
      confidence. Physical chemistry matters, and I know it when I feel it. I've learned
      from past relationships that genuine mutual attraction is important. When there's
      real chemistry and admiration, everything else works better.
    </p>

    <p>
      I'm picky, and I don't apologize for that. I've learned that being with the wrong
      person is worse than being alone. So I'm working on being the best version of
      myself I can be: growing in patience, fighting my tendency to isolate, learning to
      reach out to people despite my fear of rejection. I'm trusting that if it's God's
      will for me to find someone, He'll bring her into my life in His timing.
    </p>

    <p>
      I hope that happens. I'd love to have more children. I'd love to build a life with
      someone and have a partner in this parenting journey. But I'm holding it with open
      hands. I'm focused on being faithful with what's in front of me and loving the
      people around me well. The rest is in God's hands.
    </p>

    <h2>Daily Struggles: The Ongoing Work</h2>

    <p>
      I want to be honest about where I struggle, because this isn't a highlight reel.
    </p>

    <p>
      <strong>Self-criticism</strong>: I'm incredibly critical of myself in ways I'm not
      critical of others. I hold myself to much higher standards. I have so much empathy
      and sympathy for other people, but almost none for myself. I'm working on this.
    </p>

    <p>
      <strong>Patience</strong>: This is a fruit of the Spirit I struggle with,
      especially in situations where roles aren't clear or people are acting
      ambiguously. When I'm in a clear teaching role, I do well. Otherwise, I get
      frustrated easily.
    </p>

    <p>
      <strong>Self-control</strong>: Particularly with lust. I struggle with thinking
      about women in sexual ways, and not having a godly outlet for that makes it
      harder. Maybe that's a cop-out. I'm working on bringing this under control through
      discipline and prayer.
    </p>

    <p>
      <strong>Isolation</strong>: My default is to withdraw and assume I'm a bother.
      Reaching out to people is hard. Maintaining relationships requires consistent
      effort that doesn't come naturally to me.
    </p>

    <p>
      <strong>Fear of rejection</strong>: This underlies many of my social struggles. I
      don't want to suggest things to people and have them say no, even if they're busy.
      So I don't suggest things.
    </p>

    <p>
      These are genuine struggles. I fail at walking in love constantly. But I keep
      trying, keep praying, keep trusting that God's grace is sufficient and that my
      weakness is where His strength is made perfect.
    </p>

    <h2>What Makes Me Happy</h2>

    <p>Despite the struggles, there are things that bring me genuine happiness:</p>

    <ul class="list-disc list-inside mb-4 space-y-2">
      <li>When I solve a difficult problem at work that's been frustrating me</li>
      <li>When my kids do the right thing and make me proud</li>
      <li>When I clean the house and it looks nice</li>
      <li>
        When I feel like I've made a positive impact in someone's life, especially in my
        church community
      </li>
      <li>
        When people help me unexpectedly (like when my car battery died at church and
        multiple people came to help)
      </li>
      <li>Being able to buy a new car battery without worrying about money</li>
      <li>Quality time with friends, even if I'm terrible at initiating it</li>
      <li>Reading something that makes me see the world differently</li>
      <li>Creating something, whether it's code or music or teaching content</li>
    </ul>

    <p>
      Happiness for me isn't about accumulating things. I'm not materialistic at all. My
      books are in totes in the garage because I'm too lazy to unpack them. I like
      having nice things that are beautiful and functional, but I don't collect or
      display.
    </p>

    <p>
      Happiness is relationships, purpose, growth, and those small moments where life
      works.
    </p>

    <p>
      These daily realities (the struggles and the small joys) are where faith and work
      meet life. They're also what shape my broader vision for what I'm building toward.
    </p>
  </AboutLayout>
</template>

<script setup>
import AboutLayout from "~/components/About/AboutLayout.vue";

definePageMeta({
  middleware: 'trusted-contact'
})

useHead({
  title: "Life Now - Kenn Williamson",
  meta: [
    {
      name: "description",
      content:
        "Single dad reality, three kids with very different needs, church community, hobbies, and the daily work of walking in love.",
    },
  ],
});
</script>
