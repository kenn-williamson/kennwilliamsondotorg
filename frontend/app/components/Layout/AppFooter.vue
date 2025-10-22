<template>
  <footer class="nautical-footer">
    <div class="container mx-auto px-4 sm:px-6 lg:px-8 py-6">
      <div class="flex flex-col sm:flex-row items-center justify-between gap-4">
        <!-- Left: Copyright & GitHub -->
        <div class="flex flex-col sm:flex-row items-center gap-3">
          <div class="footer-text">
            © {{ currentYear }} Kenn Williamson. All rights reserved.
          </div>
          <a
            href="https://github.com/kenn-williamson/kennwilliamsondotorg"
            target="_blank"
            rel="noopener noreferrer"
            class="github-badge"
            aria-label="View source on GitHub"
          >
            <svg class="github-icon" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
              <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"/>
            </svg>
            <span class="github-text">Source</span>
          </a>
        </div>

        <!-- Right: Legal Links -->
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
/* Nautical footer - matches header theme */
.nautical-footer {
  position: relative;
  background: linear-gradient(to bottom, #1e293b 0%, #0f172a 100%);
  border-top: 2px solid rgba(71, 85, 105, 0.5);
  box-shadow: 0 -4px 8px rgba(0, 0, 0, 0.3);
  margin-top: auto;
}

/* Nautical geometric pattern overlay (matches header) */
.nautical-footer::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image:
    repeating-linear-gradient(
      90deg,
      transparent,
      transparent 20px,
      rgba(148, 163, 184, 0.03) 20px,
      rgba(148, 163, 184, 0.03) 40px
    ),
    repeating-linear-gradient(
      0deg,
      transparent,
      transparent 20px,
      rgba(148, 163, 184, 0.03) 20px,
      rgba(148, 163, 184, 0.03) 40px
    );
  pointer-events: none;
  z-index: 0;
}

/* Metallic gradient border effect (top border for footer) */
.nautical-footer::after {
  content: '';
  position: absolute;
  top: -2px;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(
    90deg,
    rgba(71, 85, 105, 0.5),
    rgba(59, 130, 246, 0.6),
    rgba(148, 163, 184, 0.7),
    rgba(59, 130, 246, 0.6),
    rgba(71, 85, 105, 0.5)
  );
  pointer-events: none;
}

/* Ensure content is above the pattern overlay */
.nautical-footer > div {
  position: relative;
  z-index: 1;
}

.footer-text {
  font-size: 0.875rem;
  color: #cbd5e1; /* nautical-300 */
  transition: color 0.2s;
}

.footer-link {
  font-size: 0.875rem;
  color: #e2e8f0; /* nautical-200 */
  text-decoration: none;
  transition: color 0.2s;
  position: relative;
}

.footer-link:hover {
  color: #67e8f9; /* accent-300 */
}

.footer-separator {
  color: #94a3b8; /* nautical-400 */
  font-size: 0.875rem;
}

/* GitHub badge */
.github-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.375rem 0.75rem;
  border-radius: 0.375rem;
  background: rgba(71, 85, 105, 0.3);
  border: 1px solid rgba(148, 163, 184, 0.3);
  color: #e2e8f0; /* nautical-200 */
  text-decoration: none;
  font-size: 0.875rem;
  transition: all 0.2s ease;
}

.github-badge:hover {
  background: rgba(71, 85, 105, 0.5);
  border-color: rgba(103, 232, 249, 0.5); /* accent-300 */
  color: #67e8f9; /* accent-300 */
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(103, 232, 249, 0.2);
}

.github-icon {
  width: 1.125rem;
  height: 1.125rem;
  flex-shrink: 0;
}

.github-text {
  font-weight: 500;
}

@media (max-width: 640px) {
  .footer-text,
  .footer-link {
    font-size: 0.75rem;
  }

  .github-badge {
    font-size: 0.75rem;
    padding: 0.25rem 0.5rem;
  }

  .github-icon {
    width: 1rem;
    height: 1rem;
  }
}
</style>
