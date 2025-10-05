<template>
  <div class="min-h-screen mahogany-background px-4 sm:px-6 lg:px-8 py-12">
    <div class="max-w-4xl mx-auto">
      <!-- Header -->
      <div class="mb-12">
        <h1 class="text-4xl sm:text-5xl font-bold text-gold mb-4 steampunk-heading">
          Privacy Policy
        </h1>
        <p class="text-lg text-gold-muted">
          Last Updated: October 4, 2025
        </p>
      </div>

      <!-- Content -->
      <div class="steampunk-card p-8 sm:p-12">
        <div class="prose prose-lg max-w-none steampunk-prose">

          <!-- Introduction -->
          <section class="mb-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">Introduction</h2>
            <p class="text-gray-700 leading-relaxed">
              This is a personal website operated by Kenn Williamson. I built it to share my work,
              connect with people, and demonstrate what I'm learning. I take your privacy seriously
              and want to be transparent about what data I collect and how I use it.
            </p>
            <p class="text-gray-700 leading-relaxed mt-4">
              This policy covers kennwilliamson.org and all subdomains. If you have questions,
              you can reach me at <a href="mailto:privacy@kennwilliamson.org" class="text-amber-600 hover:text-amber-700">privacy@kennwilliamson.org</a>.
            </p>
          </section>

          <!-- What Data We Collect -->
          <section class="mb-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">What Data We Collect</h2>

            <h3 class="text-xl font-semibold text-gray-800 mb-3">Account Information</h3>
            <p class="text-gray-700 leading-relaxed mb-4">
              When you create an account, I collect:
            </p>
            <ul class="list-disc pl-6 text-gray-700 space-y-2 mb-4">
              <li>Email address (required for authentication)</li>
              <li>Password (hashed using bcrypt before storage—I never see your actual password)</li>
              <li>Display name and username/slug (optional, for public profiles)</li>
              <li>Account creation and last login timestamps</li>
            </ul>

            <h3 class="text-xl font-semibold text-gray-800 mb-3">Google OAuth Data</h3>
            <p class="text-gray-700 leading-relaxed mb-4">
              If you sign in with Google, I receive:
            </p>
            <ul class="list-disc pl-6 text-gray-700 space-y-2 mb-4">
              <li>Your Google account email address</li>
              <li>Your name from your Google profile</li>
              <li>A unique identifier from Google to link your account</li>
            </ul>
            <p class="text-gray-700 leading-relaxed">
              I only request the minimum permissions necessary for authentication. I do not access
              your Gmail, Google Drive, or other Google services. The OAuth flow uses PKCE
              (Proof Key for Code Exchange) for enhanced security.
            </p>

            <h3 class="text-xl font-semibold text-gray-800 mb-3 mt-6">User-Generated Content</h3>
            <p class="text-gray-700 leading-relaxed mb-4">
              When you use site features, I store:
            </p>
            <ul class="list-disc pl-6 text-gray-700 space-y-2 mb-4">
              <li>Incident timers you create (title, description, start/end times)</li>
              <li>Phrases you submit (text and suggested tags)</li>
              <li>Any other content you choose to create through site features</li>
            </ul>

            <h3 class="text-xl font-semibold text-gray-800 mb-3 mt-6">Technical Data</h3>
            <p class="text-gray-700 leading-relaxed mb-4">
              For site functionality and security, I collect:
            </p>
            <ul class="list-disc pl-6 text-gray-700 space-y-2">
              <li>IP addresses (for rate limiting and abuse prevention)</li>
              <li>Browser and device information (user agent strings)</li>
              <li>Authentication tokens (JWT tokens stored in HTTP-only cookies)</li>
              <li>Session data (to keep you logged in)</li>
            </ul>
          </section>

          <!-- How We Use Your Data -->
          <section class="mb-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">How We Use Your Data</h2>
            <p class="text-gray-700 leading-relaxed mb-4">
              I use your data for these purposes only:
            </p>
            <ul class="list-disc pl-6 text-gray-700 space-y-2">
              <li><strong>Authentication:</strong> To verify your identity and keep you logged in</li>
              <li><strong>Site Features:</strong> To provide timers, phrases, and other functionality</li>
              <li><strong>Communication:</strong> To send important account-related messages (password resets, security alerts)</li>
              <li><strong>Security:</strong> To prevent fraud, abuse, and unauthorized access</li>
              <li><strong>Site Improvement:</strong> To understand how the site is used and fix problems</li>
            </ul>
            <p class="text-gray-700 leading-relaxed mt-4">
              <strong>I do not:</strong>
            </p>
            <ul class="list-disc pl-6 text-gray-700 space-y-2">
              <li>Sell your data to anyone</li>
              <li>Share your data with third parties for marketing</li>
              <li>Use your data for advertising or tracking across other sites</li>
              <li>Send you marketing emails (this is a personal site, not a business)</li>
            </ul>
          </section>

          <!-- How We Protect Your Data -->
          <section class="mb-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">How We Protect Your Data</h2>
            <p class="text-gray-700 leading-relaxed mb-4">
              I take reasonable security measures to protect your information:
            </p>
            <ul class="list-disc pl-6 text-gray-700 space-y-2">
              <li>All connections use HTTPS/TLS encryption</li>
              <li>Passwords are hashed using bcrypt with appropriate cost factors</li>
              <li>Authentication uses JWT tokens with secure HTTP-only cookies</li>
              <li>Database access is restricted and uses parameterized queries to prevent SQL injection</li>
              <li>The site is regularly updated to patch security vulnerabilities</li>
            </ul>
            <p class="text-gray-700 leading-relaxed mt-4">
              That said, no system is perfectly secure. While I do my best to protect your data,
              I can't guarantee absolute security. If you discover a security issue, please report
              it to <a href="mailto:security@kennwilliamson.org" class="text-amber-600 hover:text-amber-700">security@kennwilliamson.org</a>.
            </p>
          </section>

          <!-- Data Retention -->
          <section class="mb-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">Data Retention</h2>
            <p class="text-gray-700 leading-relaxed mb-4">
              I keep your data for as long as your account is active. Here's what happens in different scenarios:
            </p>
            <ul class="list-disc pl-6 text-gray-700 space-y-2">
              <li><strong>Active accounts:</strong> Data is retained indefinitely while you use the site</li>
              <li><strong>Deleted accounts:</strong> Account data is permanently deleted within 30 days, though some content you've made public (like shared timers) may remain visible but disassociated from your account</li>
              <li><strong>Technical logs:</strong> Server logs and IP addresses are retained for up to 90 days for security purposes</li>
              <li><strong>Legal compliance:</strong> If required by law, some data may be retained longer</li>
            </ul>
          </section>

          <!-- Your Rights -->
          <section class="mb-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">Your Rights (GDPR & CCPA Compliance)</h2>
            <p class="text-gray-700 leading-relaxed mb-4">
              You have the following rights regarding your data:
            </p>

            <h3 class="text-xl font-semibold text-gray-800 mb-3">Access</h3>
            <p class="text-gray-700 leading-relaxed mb-4">
              You can request a copy of all data I have about you. Contact me at
              <a href="mailto:privacy@kennwilliamson.org" class="text-amber-600 hover:text-amber-700">privacy@kennwilliamson.org</a>
              and I'll provide it within 30 days.
            </p>

            <h3 class="text-xl font-semibold text-gray-800 mb-3">Correction</h3>
            <p class="text-gray-700 leading-relaxed mb-4">
              You can update your account information directly through your profile settings.
              If you need help correcting other data, contact me.
            </p>

            <h3 class="text-xl font-semibold text-gray-800 mb-3">Deletion</h3>
            <p class="text-gray-700 leading-relaxed mb-4">
              You can delete your account at any time through your profile settings. This will
              remove your account data within 30 days. You can also request deletion by contacting me.
            </p>

            <h3 class="text-xl font-semibold text-gray-800 mb-3">Data Portability</h3>
            <p class="text-gray-700 leading-relaxed mb-4">
              You can request a machine-readable export of your data (JSON format). Contact me
              and I'll provide it within 30 days.
            </p>

            <h3 class="text-xl font-semibold text-gray-800 mb-3">Opt-Out (California Residents)</h3>
            <p class="text-gray-700 leading-relaxed mb-4">
              Under the California Consumer Privacy Act (CCPA), you have the right to opt out of
              the "sale" of your personal information. For clarity: <strong>I do not sell your data</strong>.
              I don't share it with third parties for money or other valuable consideration.
            </p>
          </section>

          <!-- Cookies and Tracking -->
          <section class="mb-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">Cookies and Tracking</h2>
            <p class="text-gray-700 leading-relaxed mb-4">
              I use cookies for essential site functionality:
            </p>
            <ul class="list-disc pl-6 text-gray-700 space-y-2">
              <li><strong>Authentication cookies:</strong> HTTP-only, secure cookies to keep you logged in</li>
              <li><strong>Session cookies:</strong> To maintain your session state across page loads</li>
              <li><strong>OAuth state cookies:</strong> Temporary cookies during Google sign-in for security (PKCE flow)</li>
            </ul>
            <p class="text-gray-700 leading-relaxed mt-4">
              <strong>I do not use:</strong>
            </p>
            <ul class="list-disc pl-6 text-gray-700 space-y-2">
              <li>Advertising cookies</li>
              <li>Third-party tracking cookies</li>
              <li>Analytics services like Google Analytics</li>
              <li>Social media tracking pixels</li>
            </ul>
            <p class="text-gray-700 leading-relaxed mt-4">
              The cookies I use are strictly necessary for the site to function. If you disable
              cookies entirely, you won't be able to log in.
            </p>
          </section>

          <!-- Third-Party Services -->
          <section class="mb-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">Third-Party Services</h2>
            <p class="text-gray-700 leading-relaxed mb-4">
              The only third-party service I use is Google OAuth for authentication. When you
              sign in with Google:
            </p>
            <ul class="list-disc pl-6 text-gray-700 space-y-2">
              <li>You're temporarily redirected to Google's servers for authentication</li>
              <li>Google's privacy policy applies during that interaction</li>
              <li>I receive only the minimal data described above (email, name, unique ID)</li>
              <li>I do not have access to your Google account beyond authentication</li>
            </ul>
            <p class="text-gray-700 leading-relaxed mt-4">
              Review Google's privacy policy at: <a href="https://policies.google.com/privacy" target="_blank" rel="noopener noreferrer" class="text-amber-600 hover:text-amber-700">https://policies.google.com/privacy</a>
            </p>
          </section>

          <!-- Children's Privacy -->
          <section class="mb-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">Children's Privacy</h2>
            <p class="text-gray-700 leading-relaxed">
              This site is not intended for children under 13. I don't knowingly collect information
              from children. If you're a parent and believe your child has created an account,
              contact me at <a href="mailto:privacy@kennwilliamson.org" class="text-amber-600 hover:text-amber-700">privacy@kennwilliamson.org</a>
              and I'll delete it immediately.
            </p>
          </section>

          <!-- International Users -->
          <section class="mb-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">International Users</h2>
            <p class="text-gray-700 leading-relaxed">
              This site is operated from the United States. If you're accessing it from outside
              the U.S., your data will be transferred to and stored in the United States. By using
              this site, you consent to that transfer. I comply with GDPR for EU users and CCPA
              for California users regardless of where you're located.
            </p>
          </section>

          <!-- Changes to This Policy -->
          <section class="mb-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">Changes to This Policy</h2>
            <p class="text-gray-700 leading-relaxed">
              I may update this policy as the site evolves. If I make significant changes, I'll
              update the "Last Updated" date at the top and notify you via email if you have an
              account. Continued use of the site after changes means you accept the updated policy.
            </p>
          </section>

          <!-- Contact -->
          <section class="mb-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">Contact Information</h2>
            <p class="text-gray-700 leading-relaxed mb-4">
              If you have questions, concerns, or requests about your privacy:
            </p>
            <p class="text-gray-700 leading-relaxed">
              <strong>Email:</strong> <a href="mailto:privacy@kennwilliamson.org" class="text-amber-600 hover:text-amber-700">privacy@kennwilliamson.org</a><br>
              <strong>Website:</strong> <a href="https://kennwilliamson.org" class="text-amber-600 hover:text-amber-700">kennwilliamson.org</a>
            </p>
          </section>

          <!-- Final Note -->
          <section>
            <div class="bg-amber-50 border-l-4 border-amber-500 p-6 rounded-r-lg">
              <p class="text-gray-700 leading-relaxed">
                <strong>A Personal Note:</strong> This is a personal project, not a corporation.
                I'm one person who genuinely cares about treating your data with respect. If you
                ever have concerns, just reach out. I'll do my best to address them promptly and honestly.
              </p>
            </div>
          </section>

        </div>
      </div>

      <!-- Navigation -->
      <div class="mt-8 text-center">
        <NuxtLink to="/" class="steampunk-link">
          ← Back to Home
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup>
useHead({
  title: 'Privacy Policy',
  meta: [
    { name: 'description', content: 'Privacy policy for KennWilliamson.org - how we collect, use, and protect your personal information.' }
  ]
})
</script>

<style scoped>
/* Mahogany Background */
.mahogany-background {
  background-color: #371200;
  background-image: url('~/assets/images/mahogany-wood.jpg');
  background-repeat: repeat;
  background-size: 400px 400px;
  background-attachment: fixed;
}

/* Steampunk Card */
.steampunk-card {
  background: linear-gradient(145deg, #8B4513 0%, #A0522D 50%, #8B4513 100%);
  border: 4px solid #C0C0C0;
  border-radius: 16px;
  box-shadow:
    inset 0 4px 8px rgba(255, 255, 255, 0.2),
    inset 0 -4px 8px rgba(0, 0, 0, 0.4),
    0 8px 32px rgba(0, 0, 0, 0.5);
}

/* Typography */
.steampunk-heading {
  font-family: 'Georgia', serif;
  color: #FFD700;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
}

.text-gold {
  color: #FFD700;
}

.text-gold-muted {
  color: #D4AF37;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
}

/* Prose Styling */
.steampunk-prose :deep(h2) {
  font-family: 'Georgia', serif;
  color: #FFD700;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
}

.steampunk-prose :deep(h3) {
  font-family: 'Georgia', serif;
  color: #D4AF37;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.steampunk-prose :deep(p),
.steampunk-prose :deep(li) {
  color: rgba(255, 255, 255, 0.9);
  font-family: 'Georgia', serif;
}

.steampunk-prose :deep(strong) {
  color: #FFD700;
}

.steampunk-prose :deep(a) {
  color: #2563eb;
  text-decoration: underline;
  transition: color 0.2s;
}

.steampunk-prose :deep(a:hover) {
  color: #1e40af;
}

.steampunk-prose :deep(ul) {
  list-style-type: disc;
}

.steampunk-prose :deep(.bg-amber-50) {
  background: linear-gradient(145deg, #5d3420 0%, #4a2c18 50%, #3c2414 100%);
  border-left: 4px solid #FFD700;
  border-radius: 0 8px 8px 0;
  padding: 24px;
}

.steampunk-prose :deep(.border-amber-500) {
  border-color: #FFD700;
}

/* Links */
.steampunk-link {
  color: #FFD700;
  font-family: 'Georgia', serif;
  font-weight: bold;
  text-decoration: none;
  transition: color 0.2s;
}

.steampunk-link:hover {
  color: #D4AF37;
}
</style>
