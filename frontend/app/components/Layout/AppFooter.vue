<template>
  <footer class="steampunk-footer">
    <div class="container mx-auto px-4 sm:px-6 lg:px-8 py-6">
      <div class="flex flex-col sm:flex-row items-center justify-between gap-4">
        <!-- Copyright -->
        <div class="footer-text">
          © {{ currentYear }} Kenn Williamson. All rights reserved.
        </div>

        <!-- Legal Links -->
        <div class="flex items-center gap-4 sm:gap-6">
          <NuxtLink to="/privacy" class="footer-link">
            Privacy Policy
          </NuxtLink>
          <span class="footer-separator">•</span>
          <NuxtLink to="/terms" class="footer-link">
            Terms of Service
          </NuxtLink>
        </div>
      </div>
    </div>
  </footer>
</template>

<script setup>
const currentYear = new Date().getFullYear()
</script>

<style scoped>
.steampunk-footer {
  background: linear-gradient(145deg, #8B4513 0%, #A0522D 50%, #8B4513 100%);
  border-top: 3px solid #C0C0C0;
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.1),
    0 -4px 8px rgba(0, 0, 0, 0.3);
  margin-top: auto;
}

.footer-text {
  font-family: 'Georgia', serif;
  font-size: 0.875rem;
  color: #D4AF37;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
}

.footer-link {
  font-family: 'Georgia', serif;
  font-size: 0.875rem;
  color: #FFD700;
  text-decoration: none;
  transition: color 0.2s;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
}

.footer-link:hover {
  color: #ffffff;
}

.footer-separator {
  color: #C0C0C0;
  font-size: 0.875rem;
}

@media (max-width: 640px) {
  .footer-text,
  .footer-link {
    font-size: 0.75rem;
  }
}
</style>
