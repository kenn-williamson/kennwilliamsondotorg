/**
 * Social Media Sharing Configuration
 *
 * Defines share images and descriptions for Open Graph and Twitter Cards
 */

export const SOCIAL_SHARE_CONFIG = {
  // Site-wide defaults
  siteName: 'KennWilliamson.org',
  baseUrl: 'https://kennwilliamson.org',
  defaultImage: '/images/homepage-share.jpg', // fallback
  twitterCard: 'summary_large_image' as const,

  // Route-specific configurations
  routes: {
    '/': {
      image: '/images/homepage-share.jpg',
      description: 'Enterprise Architect, single dad, disciple. Building software systems, navigating AI realistically, and pursuing truth, beauty, and love at the intersection of ancient wisdom and modern technology.',
    },
    '/about': {
      image: '/images/personal-share.jpg',
      description: 'Single dad to three kids, Enterprise Architect, and disciple of Christ. My story of finding purpose through faith, family, and the surprising intersection of theology and technology.',
    },
    '/about/professional': {
      image: '/images/professional-share.jpg',
      description: 'From mechanical engineering to Enterprise Architect at SEQTEK. Seven years evangelizing AI realism—the middle path between hype and skepticism—while delivering successful projects and discovering what actually works.',
    },
    '/about/ai': {
      image: '/images/professional-share.jpg', // shared with professional
      description: 'From mechanical engineering to Enterprise Architect at SEQTEK. Seven years evangelizing AI realism—the middle path between hype and skepticism—while delivering successful projects and discovering what actually works.',
    },
    '/incidents': {
      image: '/images/timer-share.jpg',
      description: 'Track incident-free time with a steampunk-themed timer. Browse community timers or create your own with real-time updates, historical data, and shareable links. Beautiful, useful, and actually built.',
    },
  },
} as const

export type ShareImageKey = 'homepage' | 'personal' | 'professional' | 'timer' | 'project'

export const SHARE_IMAGES: Record<ShareImageKey, string> = {
  homepage: '/images/homepage-share.jpg',
  personal: '/images/personal-share.jpg',
  professional: '/images/professional-share.jpg',
  timer: '/images/timer-share.jpg',
  project: '/images/project-share.jpg',
}
