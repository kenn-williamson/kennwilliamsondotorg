/**
 * Social Media Sharing Composable
 *
 * Sets Open Graph and Twitter Card meta tags using Nuxt's useSeoMeta
 */

import { SOCIAL_SHARE_CONFIG, SHARE_IMAGES, type ShareImageKey } from '~/config/social-share'

interface SocialShareOptions {
  title: string
  description?: string
  imageKey?: ShareImageKey
  customImage?: string
}

export function useSocialShare(options: SocialShareOptions) {
  const route = useRoute()

  // Get description from config or use provided description
  const routeConfig = SOCIAL_SHARE_CONFIG.routes[route.path as keyof typeof SOCIAL_SHARE_CONFIG.routes]
  const description = options.description || routeConfig?.description || ''

  // Determine image path
  let imagePath: string
  if (options.customImage) {
    imagePath = options.customImage
  } else if (options.imageKey) {
    imagePath = SHARE_IMAGES[options.imageKey]
  } else if (routeConfig?.image) {
    imagePath = routeConfig.image
  } else {
    imagePath = SOCIAL_SHARE_CONFIG.defaultImage
  }

  // Generate absolute URL for image (required by social platforms)
  // If imagePath is already absolute (starts with http:// or https://), use as-is
  // Otherwise, prepend the base URL for relative paths
  const isAbsoluteUrl = imagePath.startsWith('http://') || imagePath.startsWith('https://')
  const absoluteImageUrl = isAbsoluteUrl ? imagePath : `${SOCIAL_SHARE_CONFIG.baseUrl}${imagePath}`

  // Set SEO meta tags using Nuxt's type-safe composable
  useSeoMeta({
    // Open Graph (Facebook, LinkedIn, Discord, Slack)
    ogTitle: options.title,
    ogDescription: description,
    ogImage: absoluteImageUrl,
    ogImageAlt: options.title,
    ogImageWidth: '1200',
    ogImageHeight: '675',
    ogImageType: 'image/jpeg',
    ogType: 'website',
    ogUrl: `${SOCIAL_SHARE_CONFIG.baseUrl}${route.path}`,
    ogSiteName: SOCIAL_SHARE_CONFIG.siteName,

    // Twitter Card
    twitterCard: SOCIAL_SHARE_CONFIG.twitterCard,
    twitterTitle: options.title,
    twitterDescription: description,
    twitterImage: absoluteImageUrl,
    twitterImageAlt: options.title,
  })
}
