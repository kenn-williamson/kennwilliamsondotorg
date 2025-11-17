# Microblogging Feature - Implementation Plan

**Working Document - Not for Repository**

## Feature Overview

Personal microblog with markdown support, featured images, social sharing, and basic analytics.

### Core Requirements
- ✅ Write posts in markdown with live preview (Toast UI Editor)
- ✅ Featured/sharable image per post (1200x630px for social media)
- ✅ Admin CRUD operations (add/edit/delete)
- ✅ Public display: reverse chronological, searchable, tagged, paginated
- ✅ Draft vs Published status
- ✅ "Updated" indicator when post edited after publication
- ✅ Social sharing with proper OG/Twitter meta tags
- ✅ Analytics powered by Umami (views, shares, devices, geo, referrers)
- ✅ RSS feed
- ✅ SEO-optimized with SSR

### Out of Scope
- ❌ Embedded images/videos in markdown content
- ❌ Comments
- ❌ Multi-author support
- ❌ Categories (tags are sufficient)
- ❌ Version history (no need to store old versions, just show "edited" indicator)

---

## 📦 Technology Versions (Updated November 2025)

This plan uses the latest stable versions as of November 2025:

### Backend (Rust)
- **Rust**: 1.91.1 (stable)
- **Actix-web**: 4.11.0
- **AWS SDK for Rust**:
  - `aws-sdk-s3`: 1.112.0
  - `aws-config`: 1.8.10
- **Image processing**: image = 0.25.8
- **RSS generation**: rss = 2.0.8
- **Testing**:
  - `mockall`: 0.13.1
  - `testcontainers`: 0.25.2
  - `testcontainers-modules`: 0.11.2

### Frontend (Nuxt/Vue)
- **Nuxt.js**: 4.0.3 (already in project)
- **Toast UI Editor**: 3.2.3 (maintenance mode, stable)
  - Note: Viewer included in vue-editor package

### Infrastructure
- **PostgreSQL**: Stay on **17** via `ghcr.io/fboulnois/pg_uuidv7:1.7.0`
  - ⚠️ PostgreSQL 18 is too new (released Sept 2025) with breaking changes
  - Current: 1.6.0 → **Upgrade to 1.7.0** (still uses PG17 but adds PG18 support for future)
  - Consider PG18 upgrade in 6-12 months after stabilization
- **Umami Analytics**: v3 (Docker: `ghcr.io/umami-software/umami:latest`)
- **Docker**: Latest stable
- **Nginx**: Latest stable

**Important**:
- Rust crate version updates are safe (libraries only, no data migration)
- PostgreSQL major version upgrades require careful testing (data migration, breaking changes)
- Always test infrastructure upgrades in development first

---

## 🚨 Critical Blockers (Must Address Before Implementation)

### 1. Migration Strategy
**Issue**: No migration file structure defined
**Required**:
- Create migration in `backend/migrations/` following SQLx naming convention (`YYYYMMDDHHMMSS_create_blog_posts.sql`)
- Define or verify `update_updated_at_column()` trigger function exists
- Document rollback approach

### 2. Security - CSRF Protection
**Issue**: Admin mutation endpoints need CSRF protection verification
**Required**:
- Verify existing auth middleware includes CSRF tokens for state-changing operations
- If not present, add CSRF middleware to `/api/blog/admin/*` routes

### 3. Port Conflict - Umami
**Issue**: Umami configured for port 3000, conflicts with Nuxt dev server
**Required**:
- Assign Umami to port 3001 in `docker-compose.yml`
- Update documentation to reflect correct port

### 4. Nginx Configuration Integration
**Issue**: Umami nginx config not integrated with existing setup
**Required**:
- Specify where `/umami` location block goes in existing `nginx.conf`
- Ensure no conflicts with `/api/*` and `/backend/*` routes

### 5. AWS Credentials Security ✅ RESOLVED
**Issue**: `.env.development` is modified in git status
**Resolution**:
- ✅ Added `.env.development` to `.gitignore`
- ✅ Removed `.env.development` from git tracking (file exists locally, not in repo)
- ✅ Production uses IAM instance role (`KennWilliamsonAppRole`) - no credentials in production env
- ✅ Local dev uses cli-admin credentials (secured in .gitignore)
- ✅ Cleanup: Deleted unused `ses-email-sender` IAM user

---

## Testing Strategy & TDD Approach

### Existing Test Patterns (Follow These)

This project has ~620 tests with comprehensive coverage. The microblog feature will follow established patterns:

#### Backend Testing Patterns

**1. Repository Layer - Integration Tests with Real Database**
- Location: `backend/tests/repositories/testcontainers_*_repository_tests.rs`
- Pattern: TestContainer + real PostgreSQL
- Each test gets isolated container
- Run with: `cargo test --test testcontainers_blog_repository_tests -- --test-threads=4`

```rust
#[tokio::test]
async fn test_create_post() {
    let container = TestContainer::builder().build().await.unwrap();
    let repo = PostgresBlogRepository::new(container.pool.clone());

    let post = repo.create_post(test_data).await.unwrap();

    assert_eq!(post.title, "Expected Title");
}
```

**2. Service Layer - Unit Tests with Mocks**
- Location: `backend/src/services/*/mod.rs` (inline with `#[cfg(test)]`)
- Pattern: MockRepository (mockall)
- Fast, no database required
- Run with: `cargo test --lib blog_service`

```rust
#[tokio::test]
async fn test_create_post_handles_slug_collision() {
    let mut mock_repo = MockBlogRepository::new();

    mock_repo
        .expect_get_post_by_slug()
        .with(eq("test-post"))
        .returning(|_| Ok(Some(existing_post)));

    let service = BlogService::new(Arc::new(mock_repo), s3_client);
    // Test business logic...
}
```

**3. API Layer - Integration Tests with Full Application**
- Location: `backend/tests/api/testcontainers_*_api_tests.rs`
- Pattern: TestContext + real HTTP requests
- End-to-end auth flow
- Run with: `cargo test --test testcontainers_blog_api_tests -- --test-threads=4`

```rust
#[actix_web::test]
async fn test_create_post_success() {
    let ctx = TestContext::builder().build().await;
    let user = create_verified_user(&ctx.pool, "admin@test.com", "admin").await;
    let token = create_test_jwt_token(&user).await.unwrap();

    let resp = ctx.server.post("/api/blog/admin/posts")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&post_data)
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}
```

#### Test Utilities (Create These First)

**Test Builders** - Located in `backend/src/test_utils/`:
- `BlogPostBuilder` - Fluent API for creating test blog posts
- Pattern matches existing `UserBuilder`, `IncidentTimerBuilder`

**Mock Repositories** - Located in `backend/src/repositories/mocks/`:
- `MockBlogRepository` - Mockall-generated mock
- `MockImageStorage` - Mockall-generated mock for image operations
- Pattern matches existing mock repositories

**Data Generators** - Use existing utilities:
- `unique_test_email()` - Prevents collisions in parallel tests
- `unique_test_slug()` - Timestamp-based unique data

#### Frontend Testing Patterns

**Component Tests with Vitest**
- Location: Colocated `.spec.ts` files
- Framework: Vitest + jsdom
- Run with: `npm test -- blog`

```typescript
describe('PostCard', () => {
  it('displays post title and excerpt', () => {
    const post = { title: 'Test', excerpt: 'Excerpt' }
    const wrapper = mount(PostCard, { props: { post } })
    expect(wrapper.text()).toContain('Test')
  })
})
```

### TDD Red/Green/Refactor Workflow

**For each feature:**

1. **RED**: Write failing test first
   - Repository test: Integration with real database
   - Service test: Unit with mocks
   - API test: Integration with real HTTP

2. **GREEN**: Implement minimum code to pass
   - Don't over-engineer
   - Get to green quickly
   - Ensure test passes

3. **REFACTOR**: Clean up code
   - Extract helpers
   - Improve naming
   - Optimize queries
   - **Tests still pass**

**Critical**: Run with `--test-threads=4` to prevent Docker resource exhaustion on integration tests.

---

## Database Schema

### `blog_posts` Table

```sql
CREATE TABLE blog_posts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    slug VARCHAR(255) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    excerpt TEXT,  -- Manual or auto-generated from content
    content TEXT NOT NULL,  -- Markdown content
    featured_image_url TEXT,  -- S3 URL
    featured_image_alt TEXT,  -- Accessibility
    status VARCHAR(20) NOT NULL DEFAULT 'draft',  -- 'draft' | 'published'
    tags TEXT[] DEFAULT '{}',  -- PostgreSQL array
    published_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    -- SEO metadata
    meta_description TEXT,  -- Falls back to excerpt if null

    -- Search
    search_vector tsvector GENERATED ALWAYS AS (
        setweight(to_tsvector('english', coalesce(title, '')), 'A') ||
        setweight(to_tsvector('english', coalesce(excerpt, '')), 'B') ||
        setweight(to_tsvector('english', coalesce(content, '')), 'C')
    ) STORED
);

-- Indexes
CREATE INDEX idx_blog_posts_status ON blog_posts(status);
CREATE INDEX idx_blog_posts_published_at ON blog_posts(published_at DESC);
CREATE INDEX idx_blog_posts_slug ON blog_posts(slug);
CREATE INDEX idx_blog_posts_tags ON blog_posts USING GIN(tags);
CREATE INDEX idx_blog_posts_search ON blog_posts USING GIN(search_vector);

-- Trigger for updated_at
CREATE TRIGGER update_blog_posts_updated_at
    BEFORE UPDATE ON blog_posts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

### Analytics

**Analytics are handled by Umami** - no custom analytics tables needed.

Umami manages its own database schema for:
- Page views and unique visitors
- Custom events (share clicks, etc.)
- Referrer tracking
- Device/browser/geo data

See "Analytics with Umami" section below for integration details.

---

## Image Storage (S3 Implementation)

### Architecture Overview

**ImageStorage Trait Abstraction** (see Service Layer section for trait definition):
- BlogService depends on `ImageStorage` trait (not S3 directly)
- `S3ImageStorage` is one implementation of the trait
- URL-driven design (no separate images database table)
- Easily testable with `MockImageStorage`

**Why This Design:**
- ✅ BlogService doesn't know about S3 internals
- ✅ Can swap S3 → CloudFront → local storage without changing service
- ✅ Unit tests use mock without real S3 uploads
- ✅ Migration path to media library documented in trait

### S3 Bucket Configuration

**Bucket Setup:**
- Bucket name: `kennwilliamson-blog-images`
- Region: `us-east-1` (same as your deployment)
- Public read access: Yes (for direct URL access)
- Lifecycle: Keep all images (no expiration)
- Optional: CloudFront CDN for faster delivery

**Directory Structure:**
```
s3://kennwilliamson-blog-images/
  blog/
    featured/
      {uuid}.jpg  -- Optimized featured images (1200x630px)
    originals/
      {uuid}.jpg  -- Original uploads (for future re-processing)
```

### S3ImageStorage Implementation

**Upload Flow:**
1. Admin uploads image via Nuxt form
2. Frontend sends to `/api/blog/admin/upload-image` (multipart/form-data)
3. Route handler calls `image_storage.upload_image(data, filename)`
4. S3ImageStorage implementation:
   - **Validates file size** (<5MB) - Prevent large uploads
   - **Sanitizes filename** - Remove path traversal (`../`, etc.)
   - **Validates MIME type** - Verify image format by loading with `image` crate (not just extension)
   - **Generates UUID** - Prevent filename conflicts/collisions
   - Saves original to S3 `originals/{uuid}.jpg`
   - Resizes to 1200x630px (social media optimal)
   - Optimizes/compresses (80% quality JPEG)
   - Saves to S3 `featured/{uuid}.jpg`
   - **Builds public URLs** - Returns full S3 URLs (not just keys)
   - Returns `ImageUrls { featured_url, original_url }`
5. Route handler returns URLs to frontend
6. Frontend stores featured URL in post form
7. BlogService stores URL (string) in `blog_posts.featured_image_url`

**Security Validations:**
```rust
// 1. File size check
if image_data.len() > 5 * 1024 * 1024 {
    bail!("Image exceeds 5MB limit");
}

// 2. Sanitize filename (prevent path traversal)
fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-')
        .collect()
}

// 3. Validate actual image format (prevents malicious files)
let img = image::load_from_memory(&image_data)
    .context("Invalid image format")?; // Will fail if not a real image

// 4. Use UUID for storage (prevent injection)
let image_id = Uuid::new_v4();
let key = format!("blog/featured/{}.jpg", image_id);
```

**Rust Dependencies:**
```toml
# Cargo.toml additions (Latest versions as of November 2025)
aws-sdk-s3 = "1.112"
aws-config = { version = "1.8", features = ["behavior-version-latest"] }
image = "0.25"  # For resizing/optimization (latest: 0.25.8)
tokio = { version = "1", features = ["full"] }
```

**Image Specs:**
- Format: JPEG (better compression than PNG for photos)
- Dimensions: 1200x630px (Facebook/Twitter/LinkedIn optimal)
- Quality: 80% (good balance of quality/size)
- Max file size: ~300KB after optimization

---

## Analytics with Umami

### Overview

**Umami** is a privacy-focused, self-hosted analytics platform that handles all website and blog analytics.

**Current Version:** Umami v3 (latest major version as of November 2025)

**Why Umami:**
- ✅ Privacy-compliant (GDPR/CCPA) - no cookies, anonymized data
- ✅ Self-hosted - data stays in your PostgreSQL database
- ✅ Lightweight - ~150MB memory footprint, ~2KB tracking script
- ✅ Comprehensive - page views, visitors, devices, geo, referrers, custom events
- ✅ Battle-tested - don't reinvent analytics logic
- ✅ Professional dashboard - real-time stats, charts, exports (v3 has improved UI)

### Docker Compose Setup

Add Umami service to `docker-compose.yml`:

```yaml
services:
  umami:
    # Umami v3 (latest as of November 2025)
    # Changed from 'postgresql-latest' to 'latest' (tag naming changed)
    image: ghcr.io/umami-software/umami:latest
    container_name: kennwilliamsondotorg-umami-1
    ports:
      - "3001:3000"  # External port 3001 to avoid conflict with Nuxt (internal still 3000)
    environment:
      DATABASE_URL: postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres:5432/${POSTGRES_DB}
      DATABASE_TYPE: postgresql
      APP_SECRET: ${UMAMI_APP_SECRET}  # Random string for session encryption
      CORS_ALLOWED_ORIGINS: https://kennwilliamson.org
    depends_on:
      - postgres
    restart: unless-stopped
    networks:
      - kennwilliamson-network
```

**Nginx configuration** for `/umami` path:

```nginx
# In nginx.conf
location /umami/ {
    proxy_pass http://umami:3000/;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
}
```

### Event Tracking

#### Frontend Integration

Add Umami tracking script to Nuxt layout:

```vue
<!-- frontend/app/app.vue or layouts/default.vue -->
<script setup>
useHead({
  script: [
    {
      src: '/umami/script.js',
      'data-website-id': '{{UMAMI_WEBSITE_ID}}', // Get from Umami dashboard
      async: true,
      defer: true
    }
  ]
})
</script>
```

#### Track Page Views (Automatic)

Umami automatically tracks page views on route changes.

#### Track Custom Events

```vue
<!-- components/blog/ShareButtons.vue -->
<script setup>
const trackShare = (platform: string) => {
  // Track share event with metadata
  if (window.umami) {
    window.umami.track('blog-share', {
      platform,
      post_id: props.postId,
      post_slug: props.slug
    })
  }

  // Open share window...
}
</script>

<template>
  <button @click="trackShare('twitter')">Share on Twitter</button>
  <button @click="trackShare('facebook')">Share on Facebook</button>
  <button @click="trackShare('linkedin')">Share on LinkedIn</button>
</template>
```

#### Track Blog Post Views

```vue
<!-- pages/blog/[slug].vue -->
<script setup>
onMounted(() => {
  // Track blog post view with metadata
  if (window.umami) {
    window.umami.track('blog-view', {
      post_id: post.value.id,
      post_slug: post.value.slug,
      tags: post.value.tags.join(',')
    })
  }
})
</script>
```

### Querying Stats (Optional)

If you need to display stats in your app (e.g., view counts on blog posts):

#### Option 1: Query Umami API (Rust Backend)

```rust
// backend/src/services/umami_service.rs
use reqwest::Client;

pub struct UmamiService {
    base_url: String,
    api_key: String,
    client: Client,
}

impl UmamiService {
    pub async fn get_event_stats(
        &self,
        event_name: &str,
        filter: &str, // e.g., "post_id:123"
    ) -> Result<EventStats, Error> {
        let url = format!(
            "{}/api/websites/{}/events",
            self.base_url,
            self.website_id
        );

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .query(&[
                ("name", event_name),
                ("filter", filter),
            ])
            .send()
            .await?;

        Ok(response.json().await?)
    }
}
```

#### Option 2: Query Umami Database Directly

```rust
// backend/src/repositories/postgres/umami_repository.rs
pub async fn get_post_view_count(&self, post_id: Uuid) -> Result<i64, Error> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM umami.event_data
        WHERE website_id = $1
          AND event_name = 'blog-view'
          AND event_data->>'post_id' = $2
        "#,
        self.website_id,
        post_id.to_string()
    )
    .fetch_one(&self.pool)
    .await?;

    Ok(count)
}
```

### Admin Dashboard Access

**Access Umami dashboard:**
- URL: `https://kennwilliamson.org/umami`
- Login with admin credentials (set during first setup)
- View real-time stats, events, visitor insights
- Export data, create reports

**Embed dashboard in admin panel (optional):**

```vue
<!-- pages/admin/analytics.vue -->
<template>
  <div class="analytics-dashboard">
    <h1>Blog Analytics</h1>
    <iframe
      src="https://kennwilliamson.org/umami/share/{{SHARE_TOKEN}}"
      width="100%"
      height="800px"
      frameborder="0"
    />
  </div>
</template>
```

### Umami Database Schema

Umami automatically creates these tables in your PostgreSQL database:
- `umami.website` - Websites being tracked
- `umami.session` - User sessions
- `umami.event` - Page views and events
- `umami.event_data` - Custom event metadata (JSON)

**No manual migration needed** - Umami handles its own schema.

---

## Backend API (Rust/Actix)

### Endpoints

#### Public Endpoints (No Auth)
```
GET  /api/blog/posts
     Query params: ?page=1&limit=10&tag=rust&status=published
     Returns: { posts: [...], total: 100, page: 1, total_pages: 10 }

GET  /api/blog/posts/:slug
     Returns: Single post with content

GET  /api/blog/search
     Query params: ?q=keyword&page=1&limit=10
     Returns: { posts: [...], total: 50 }

GET  /api/blog/tags
     Returns: { tags: ['rust', 'nuxt', ...], counts: {...} }

GET  /api/blog/feed.xml
     Returns: RSS feed (application/rss+xml)
```

#### Admin Endpoints (Requires Auth + Admin Role)
```
POST   /api/blog/posts
       Body: { title, slug, content, excerpt, featured_image_url, tags, status, meta_description }
       Returns: Created post

PUT    /api/blog/posts/:id
       Body: Same as POST
       Returns: Updated post

DELETE /api/blog/posts/:id
       Returns: 204 No Content

POST   /api/blog/upload-image
       Body: multipart/form-data with 'image' field
       Returns: { url: 's3://...', original_url: 's3://...' }
```

**Note:** Analytics tracking handled by Umami (frontend-only). No backend endpoints needed for analytics.
```

### Repository Layer

**New file:** `backend/src/repositories/traits/blog_repository.rs`

```rust
#[async_trait]
pub trait BlogRepository: Send + Sync {
    async fn create_post(&self, post: CreateBlogPost) -> Result<BlogPost, Error>;
    async fn get_post_by_id(&self, id: Uuid) -> Result<Option<BlogPost>, Error>;
    async fn get_post_by_slug(&self, slug: &str) -> Result<Option<BlogPost>, Error>;
    async fn list_posts(&self, filters: BlogPostFilters) -> Result<BlogPostList, Error>;
    async fn update_post(&self, id: Uuid, post: UpdateBlogPost) -> Result<BlogPost, Error>;
    async fn delete_post(&self, id: Uuid) -> Result<(), Error>;
    async fn search_posts(&self, query: &str, page: i32, limit: i32) -> Result<BlogPostList, Error>;
    async fn get_all_tags(&self, status: Option<String>) -> Result<Vec<TagCount>, Error>;
}
```

**New file:** `backend/src/repositories/postgres/blog_repository.rs`

Implements above trait with SQLx queries.

### Service Layer

**Architecture Decision: ImageStorage Abstraction**

Instead of directly coupling BlogService to AWS S3, we use dependency injection with an `ImageStorage` trait:
- ✅ **Testable**: Mock image storage in unit tests
- ✅ **Flexible**: Can swap storage backends without changing BlogService
- ✅ **Clean**: Single responsibility - BlogService handles blog logic, ImageStorage handles images

**New file:** `backend/src/repositories/traits/image_storage.rs`

```rust
#[async_trait]
pub trait ImageStorage: Send + Sync {
    async fn upload_image(&self, image_data: Vec<u8>, filename: String) -> Result<ImageUrls>;
    async fn delete_image(&self, url: &str) -> Result<()>;
}

pub struct ImageUrls {
    pub featured_url: String,
    pub original_url: String,
}
```

**New file:** `backend/src/repositories/s3_image_storage.rs`

```rust
pub struct S3ImageStorage {
    s3_client: aws_sdk_s3::Client,
    bucket_name: String,
}

impl S3ImageStorage {
    // Implements security validations, image processing, S3 upload
}
```

**New file:** `backend/src/repositories/mocks/mock_image_storage.rs`

```rust
// Mockall-generated mock for unit tests
mock! {
    pub ImageStorage {}

    #[async_trait]
    impl ImageStorage for ImageStorage {
        async fn upload_image(&self, image_data: Vec<u8>, filename: String) -> Result<ImageUrls>;
        async fn delete_image(&self, url: &str) -> Result<()>;
    }
}
```

**New file:** `backend/src/services/blog_service.rs`

```rust
pub struct BlogService {
    blog_repo: Arc<dyn BlogRepository>,
    image_storage: Arc<dyn ImageStorage>,  // ✅ Injected abstraction, not direct S3 coupling
}

impl BlogService {
    // CRUD operations
    pub async fn create_post(&self, request: CreateBlogPostRequest) -> Result<BlogPost, Error> {
        // Auto-generate slug if not provided
        let base_slug = request.slug.unwrap_or_else(|| slugify(&request.title));

        // Handle slug collisions (auto-append numeric suffix)
        let slug = self.ensure_unique_slug(&base_slug).await?;

        // Auto-generate excerpt if not provided (first 160 chars, strip markdown)
        let excerpt = request.excerpt.or_else(|| Some(generate_excerpt(&request.content)));

        // Set published_at if status is 'published'
        let published_at = if request.status == "published" {
            Some(Utc::now())
        } else {
            None
        };

        self.blog_repo.create_post(CreateBlogPost {
            slug,
            title: request.title,
            content: request.content,
            excerpt,
            status: request.status,
            tags: request.tags,
            featured_image_url: request.featured_image_url,
            meta_description: request.meta_description,
        }).await
    }

    async fn ensure_unique_slug(&self, base_slug: &str) -> Result<String> {
        let mut slug = base_slug.to_string();
        let mut counter = 2;

        // Keep appending numbers until we find unique slug
        while self.blog_repo.get_post_by_slug(&slug).await?.is_some() {
            slug = format!("{}-{}", base_slug, counter);
            counter += 1;
        }

        Ok(slug)
    }

    pub async fn delete_post(&self, id: Uuid) -> Result<()> {
        // Get post to find image URLs before deleting
        let post = self.blog_repo.get_post_by_id(id).await?
            .ok_or_else(|| anyhow!("Post not found"))?;

        // Delete from database
        self.blog_repo.delete_post(id).await?;

        // Clean up images immediately via ImageStorage abstraction
        if let Some(featured_url) = post.featured_image_url {
            self.image_storage.delete_image(&featured_url).await?;
        }

        Ok(())
    }

    pub async fn publish_post(&self, id: Uuid) -> Result<BlogPost, Error> {
        // Update status to published, set published_at
    }
}
}

fn generate_excerpt(content: &str) -> String {
    // Strip markdown formatting
    let plain_text = strip_markdown(content);
    // Take first 160 characters, break at word boundary
    truncate_words(&plain_text, 160)
}

fn slugify(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
```

### Routes

**New file:** `backend/src/routes/blog.rs`

```rust
pub fn configure_blog_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blog")
            // Public routes
            .route("/posts", web::get().to(list_posts))
            .route("/posts/{slug}", web::get().to(get_post))
            .route("/search", web::get().to(search_posts))
            .route("/tags", web::get().to(get_tags))
            .route("/feed.xml", web::get().to(rss_feed))

            // Admin routes (protected)
            .service(
                web::scope("/admin")
                    .wrap(RequireAuth)
                    .wrap(RequireRole::new(vec!["admin".to_string()]))
                    .route("/posts", web::post().to(create_post))
                    .route("/posts/{id}", web::put().to(update_post))
                    .route("/posts/{id}", web::delete().to(delete_post))
                    .route("/upload-image", web::post().to(upload_image))
            )
    );
}
```

---

## Frontend (Nuxt.js)

### Pages

**Public Pages:**
```
pages/blog/index.vue         -- Blog list (SSR for SEO)
pages/blog/[slug].vue        -- Single post (SSR for SEO)
pages/blog/search.vue        -- Search results
pages/blog/tag/[tag].vue     -- Posts by tag
```

**Admin Pages:**
```
pages/admin/blog/index.vue       -- Blog management (list all posts)
pages/admin/blog/new.vue         -- Create new post
pages/admin/blog/[id]/edit.vue   -- Edit existing post
```

### Components

```
components/blog/PostCard.vue           -- Post preview card (for list view)
components/blog/PostContent.vue        -- Markdown renderer (Toast UI Viewer)
components/blog/ShareButtons.vue       -- Social share buttons
components/blog/ImageUpload.vue        -- Featured image uploader
components/blog/MarkdownEditor.vue     -- Markdown editor with live preview (Toast UI Editor)
components/blog/TagInput.vue           -- Tag selection/creation
components/blog/SearchBar.vue          -- Search input
components/blog/Pagination.vue         -- Pagination controls
components/blog/AnalyticsWidget.vue    -- View/share stats (admin)
```

### Public Blog List (`pages/blog/index.vue`)

```vue
<script setup>
const route = useRoute()
const page = computed(() => parseInt(route.query.page) || 1)
const tag = computed(() => route.query.tag)

const { data: postsData } = await useAsyncData(
  `blog-posts-${page.value}-${tag.value}`,
  () => $fetch('/api/blog/posts', {
    params: {
      page: page.value,
      limit: 10,
      tag: tag.value,
      status: 'published'
    }
  })
)

useHead({
  title: 'Blog - Kenn Williamson',
  meta: [
    { name: 'description', content: 'Thoughts on Christian Voluntarism, technology, and personal growth' }
  ]
})
</script>

<template>
  <div class="blog-list">
    <h1>Blog</h1>
    <BlogSearchBar />

    <div class="posts">
      <BlogPostCard
        v-for="post in postsData.posts"
        :key="post.id"
        :post="post"
      />
    </div>

    <BlogPagination
      :current="postsData.page"
      :total="postsData.total_pages"
    />
  </div>
</template>
```

### Single Post (`pages/blog/[slug].vue`)

```vue
<script setup>
const route = useRoute()
const { data: post } = await useAsyncData(
  `blog-post-${route.params.slug}`,
  () => $fetch(`/api/blog/posts/${route.params.slug}`)
)

if (!post.value) {
  throw createError({ statusCode: 404, message: 'Post not found' })
}

// Track blog post view with Umami
onMounted(() => {
  if (window.umami) {
    window.umami.track('blog-view', {
      post_id: post.value.id,
      post_slug: post.value.slug,
      tags: post.value.tags.join(',')
    })
  }
})

// SEO meta tags
const postUrl = computed(() => `https://kennwilliamson.org/blog/${post.value.slug}`)

// Check if post was edited after publication
const wasEdited = computed(() => {
  if (!post.value.published_at || !post.value.updated_at) return false
  const publishTime = new Date(post.value.published_at).getTime()
  const updateTime = new Date(post.value.updated_at).getTime()
  // Consider "edited" if updated more than 1 hour after publish
  return (updateTime - publishTime) > (60 * 60 * 1000)
})

const formatDate = (dateString) => {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  })
}

useHead({
  title: post.value.title,
  meta: [
    { name: 'description', content: post.value.meta_description || post.value.excerpt },

    // OpenGraph
    { property: 'og:title', content: post.value.title },
    { property: 'og:description', content: post.value.excerpt },
    { property: 'og:image', content: post.value.featured_image_url },
    { property: 'og:url', content: postUrl.value },
    { property: 'og:type', content: 'article' },
    { property: 'article:published_time', content: post.value.published_at },

    // Twitter Card
    { name: 'twitter:card', content: 'summary_large_image' },
    { name: 'twitter:title', content: post.value.title },
    { name: 'twitter:description', content: post.value.excerpt },
    { name: 'twitter:image', content: post.value.featured_image_url },
  ]
})
</script>

<template>
  <article class="blog-post">
    <img
      v-if="post.featured_image_url"
      :src="post.featured_image_url"
      :alt="post.featured_image_alt"
      class="featured-image"
    />

    <header>
      <h1>{{ post.title }}</h1>
      <div class="meta">
        <div class="dates">
          <time :datetime="post.published_at">
            {{ formatDate(post.published_at) }}
          </time>
          <!-- Show "Updated" if edited after publication -->
          <time
            v-if="wasEdited"
            :datetime="post.updated_at"
            class="edited-indicator"
          >
            • Updated {{ formatDate(post.updated_at) }}
          </time>
        </div>
        <div class="tags">
          <NuxtLink
            v-for="tag in post.tags"
            :key="tag"
            :to="`/blog?tag=${tag}`"
          >
            #{{ tag }}
          </NuxtLink>
        </div>
      </div>
    </header>

    <BlogPostContent :markdown="post.content" />

    <footer>
      <BlogShareButtons
        :title="post.title"
        :url="postUrl"
        :post-id="post.id"
      />
    </footer>
  </article>
</template>
```

### Admin Post Editor (`pages/admin/blog/new.vue` and `[id]/edit.vue`)

```vue
<script setup>
const route = useRoute()
const isEdit = computed(() => !!route.params.id)

const postForm = ref({
  title: '',
  slug: '',
  excerpt: '',
  content: '',
  featured_image_url: '',
  featured_image_alt: '',
  tags: [],
  status: 'draft',
  meta_description: ''
})

// Load existing post if editing
if (isEdit.value) {
  const { data: post } = await useAsyncData(
    `blog-post-edit-${route.params.id}`,
    () => $fetch(`/api/blog/posts/${route.params.id}`)
  )
  postForm.value = { ...post.value }
}

// Auto-generate slug from title
watch(() => postForm.value.title, (newTitle) => {
  if (!isEdit.value) {
    postForm.value.slug = newTitle
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, '-')
      .replace(/^-|-$/g, '')
  }
})

const savePost = async (publish = false) => {
  if (publish) postForm.value.status = 'published'

  const endpoint = isEdit.value
    ? `/api/blog/posts/${route.params.id}`
    : '/api/blog/posts'

  const method = isEdit.value ? 'PUT' : 'POST'

  const savedPost = await $fetch(endpoint, {
    method,
    body: postForm.value
  })

  navigateTo(`/admin/blog`)
}
</script>

<template>
  <div class="blog-editor">
    <h1>{{ isEdit ? 'Edit Post' : 'New Post' }}</h1>

    <form @submit.prevent="savePost(false)">
      <input v-model="postForm.title" placeholder="Title" required />
      <input v-model="postForm.slug" placeholder="URL slug" required />
      <textarea v-model="postForm.excerpt" placeholder="Excerpt" rows="3" />

      <BlogImageUpload v-model="postForm.featured_image_url" />
      <input v-model="postForm.featured_image_alt" placeholder="Image alt text" />

      <BlogMarkdownEditor v-model="postForm.content" />

      <BlogTagInput v-model="postForm.tags" />

      <textarea v-model="postForm.meta_description" placeholder="Meta description (SEO)" rows="2" />

      <div class="actions">
        <button type="submit">Save Draft</button>
        <button type="button" @click="savePost(true)">Publish</button>
      </div>
    </form>
  </div>
</template>
```

### Markdown Components

#### Install Dependencies

```bash
# Toast UI packages (editor + viewer for consistent rendering)
# Latest version: 3.2.3 (maintained but not actively developed, last update 3 years ago)
# Note: Viewer is included in vue-editor package, no separate vue-viewer package exists
npm install @toast-ui/vue-editor@3.2.3
```

**Important Note:** Toast UI Vue Editor is in maintenance mode but still functional and stable. The Viewer component is imported from `@toast-ui/vue-editor`, not a separate package.

#### Markdown Renderer (Public Pages)

```vue
<!-- components/blog/PostContent.vue -->
<script setup>
import { Viewer } from '@toast-ui/vue-viewer'
import '@toast-ui/editor/dist/toastui-editor-viewer.css'

const props = defineProps(['markdown'])
</script>

<template>
  <div class="markdown-content">
    <Viewer :initial-value="props.markdown" />
  </div>
</template>

<style>
/* Optional: Override Toast UI viewer styles to match your theme */
.markdown-content {
  /* Add custom styles if needed */
}
</style>
```

#### Markdown Editor (Admin Pages)

```vue
<!-- components/blog/MarkdownEditor.vue -->
<script setup>
import { Editor } from '@toast-ui/vue-editor'
import '@toast-ui/editor/dist/toastui-editor.css'
import { useDebounceFn } from '@vueuse/core'

const content = defineModel()
const editorRef = ref(null)

// Auto-save with debounce (user requirement: save on every change, debounced)
const emit = defineEmits(['save'])

const debouncedSave = useDebounceFn(() => {
  if (content.value) {
    emit('save', content.value)
  }
}, 2000) // Save 2 seconds after user stops typing

const handleEditorLoad = (editor) => {
  editorRef.value = editor
}

const handleChange = () => {
  if (editorRef.value) {
    content.value = editorRef.value.getMarkdown()
    // Trigger auto-save on every change
    debouncedSave()
  }
}

// Mobile responsive: switch to tabbed mode on small screens
const previewStyle = computed(() => {
  if (typeof window === 'undefined') return 'vertical'
  return window.innerWidth < 768 ? 'tab' : 'vertical'
})
</script>

<template>
  <div class="markdown-editor">
    <Editor
      :initial-value="content"
      initial-edit-type="markdown"
      :preview-style="previewStyle"
      height="600px"
      @load="handleEditorLoad"
      @change="handleChange"
    />
  </div>
</template>

<style scoped>
.markdown-editor {
  border: 1px solid var(--border-color);
  border-radius: 4px;
  overflow: hidden;
}
</style>
```

**Why Toast UI for Both Editor + Viewer:**
- **Consistency:** Editor preview matches exactly what readers see on public pages
- **Single rendering engine:** One markdown parser, one set of styles
- **GFM support:** Both use GitHub Flavored Markdown
- **Syntax highlighting:** Consistent code block rendering everywhere

**Toast UI Editor Features:**
- Split-view: Markdown editor on left, live preview on right
- Toolbar with common markdown operations (bold, italic, headings, lists, links, images, code blocks)
- Keyboard shortcuts for markdown syntax

---

## RSS Feed

Generate standard RSS 2.0 feed at `/api/blog/feed.xml`

**Rust Dependency:**
```toml
# Cargo.toml (Latest version as of November 2025)
rss = "2.0"  # Use proper RSS library instead of manual XML (latest: 2.0.8, compatible with 2.0)
```

**Why RSS Library:**
- ✅ Handles XML escaping automatically
- ✅ Proper CDATA sections for content
- ✅ Correct date formatting (RFC 2822)
- ✅ Prevents malformed XML
- ❌ Manual string concatenation is error-prone and doesn't handle special characters

**Backend route handler:**

```rust
use rss::{Channel, ChannelBuilder, Item, ItemBuilder};

async fn rss_feed(blog_service: web::Data<Arc<BlogService>>) -> Result<HttpResponse, Error> {
    let posts = blog_service.list_posts(BlogPostFilters {
        status: Some("published".to_string()),
        limit: 50,
        ..Default::default()
    }).await?;

    let items: Vec<Item> = posts.posts.into_iter().map(|post| {
        ItemBuilder::default()
            .title(Some(post.title))
            .link(Some(format!("https://kennwilliamson.org/blog/{}", post.slug)))
            .description(Some(post.excerpt.unwrap_or_default()))
            .pub_date(post.published_at.map(|dt| dt.to_rfc2822()))
            .guid(Some(rss::Guid {
                value: format!("https://kennwilliamson.org/blog/{}", post.slug),
                permalink: true,
            }))
            .build()
    }).collect();

    let channel = ChannelBuilder::default()
        .title("Kenn Williamson's Blog")
        .link("https://kennwilliamson.org/blog")
        .description("Thoughts on Christian Voluntarism, technology, and personal growth")
        .items(items)
        .build();

    Ok(HttpResponse::Ok()
        .content_type("application/rss+xml; charset=utf-8")
        .body(channel.to_string()))
}
```

---

## Implementation Phases (TDD Red/Green Workflow)

### Phase 0: Test Infrastructure (Create Before Code)

**Goal**: Set up all test utilities before writing business logic

#### 0.1 Database Migration
1. Create migration: `backend/migrations/YYYYMMDDHHMMSS_create_blog_posts.sql`
2. Define `update_updated_at_column()` trigger function (or verify exists from previous migrations)
3. Add `blog_posts` table with all indexes
4. Run migration: `./scripts/setup-db.sh`

**Validation**:
```bash
psql -U kennwilliamson_user -d kennwilliamson -c "\d blog_posts"
# Should show table with all columns and indexes
```

#### 0.2 Test Builders
**File**: `backend/src/test_utils/blog_post_builder.rs`

```rust
pub struct BlogPostBuilder {
    id: Option<Uuid>,
    slug: Option<String>,
    title: Option<String>,
    content: Option<String>,
    status: Option<String>,
    tags: Vec<String>,
    // ... all fields
}

impl BlogPostBuilder {
    pub fn new() -> Self {
        // UUID-based defaults for unique data
        let uuid = Uuid::new_v4();
        Self {
            slug: Some(format!("test-post-{}", uuid)),
            title: Some(format!("Test Post {}", uuid)),
            content: Some("# Test Content".into()),
            status: Some("draft".into()),
            tags: vec![],
            // ...
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn published(mut self) -> Self {
        self.status = Some("published".into());
        self
    }

    pub fn build(self) -> BlogPost {
        // Construct BlogPost with defaults
    }

    pub async fn persist(self, pool: &PgPool) -> Result<BlogPost> {
        // INSERT into blog_posts, return BlogPost
    }
}
```

**Validation**: `cargo build` → compiles successfully

#### 0.3 Mock Repository
**File**: `backend/src/repositories/mocks/mock_blog_repository.rs`

```rust
use mockall::mock;

mock! {
    pub BlogRepository {}

    #[async_trait]
    impl BlogRepository for BlogRepository {
        async fn create_post(&self, post: CreateBlogPost) -> Result<BlogPost>;
        async fn get_post_by_id(&self, id: Uuid) -> Result<Option<BlogPost>>;
        async fn get_post_by_slug(&self, slug: &str) -> Result<Option<BlogPost>>;
        // ... all trait methods from BlogRepository
    }
}
```

**Validation**: `cargo build` → compiles successfully

#### 0.4 ImageStorage Trait & Mock
**Files**:
- `backend/src/repositories/traits/image_storage.rs` - Trait definition
- `backend/src/repositories/mocks/mock_image_storage.rs` - Mockall mock

```rust
// ImageStorage trait (URL-driven, no database IDs)
#[async_trait]
pub trait ImageStorage: Send + Sync {
    async fn upload_image(&self, image_data: Vec<u8>, filename: String) -> Result<ImageUrls>;
    async fn delete_image(&self, url: &str) -> Result<()>;
}

pub struct ImageUrls {
    pub featured_url: String,
    pub original_url: String,
}

// MockImageStorage for unit tests
mock! {
    pub ImageStorage {}

    #[async_trait]
    impl ImageStorage for ImageStorage {
        async fn upload_image(&self, image_data: Vec<u8>, filename: String) -> Result<ImageUrls>;
        async fn delete_image(&self, url: &str) -> Result<()>;
    }
}
```

**Validation**:
- `cargo build` → compiles successfully
- `cargo test mock_image_storage` → 6 tests pass

---

### Phase 1: S3 + Umami Setup (Infrastructure)

#### 1.1 AWS S3 Configuration ✅ COMPLETE

- [x] Create S3 bucket: `kennwilliamson-blog-images`
- [x] Set bucket policy for public read access
- [x] CORS configuration for web uploads (kennwilliamson.org, localhost)
- [x] IAM permissions configured:
  - **Production EC2**: IAM instance role `KennWilliamsonAppRole` with `BlogImagesS3Access` policy
  - **Local dev**: cli-admin user credentials (already in .env.development)
- [x] Add to `.env.development`:
   ```bash
   AWS_REGION=us-east-1  # Already present
   AWS_S3_BUCKET_BLOG_IMAGES=kennwilliamson-blog-images  # Added
   AWS_ACCESS_KEY_ID=<cli-admin-credentials>  # Already present
   AWS_SECRET_ACCESS_KEY=<cli-admin-credentials>  # Already present
   ```
- [x] Verify `.env.development` is in `.gitignore` (added and removed from git tracking)
- [x] Cleanup: Deleted unused `ses-email-sender` IAM user (never used, had no access keys)
- [ ] Add Cargo dependencies (latest versions as of November 2025):
   ```toml
   # AWS SDK (latest stable versions)
   aws-sdk-s3 = "1.112"
   aws-config = { version = "1.8", features = ["behavior-version-latest"] }

   # Image processing
   image = "0.25"  # Latest: 0.25.8

   # RSS feed generation
   rss = "2.0"  # Latest: 2.0.8

   # Already in project (verify versions):
   actix-web = "4.11"  # Latest stable
   tokio = { version = "1", features = ["full"] }
   ```

**Validation**: ✅ PASSED
- ✅ Bucket exists and is accessible
- ✅ Public read access verified (curl test passed)
- ✅ Upload permissions verified (test file upload successful with cli-admin)
- ✅ CORS configuration verified (kennwilliamson.org, localhost, localhost:3000)
- ✅ EC2 instance role has S3 permissions (BlogImagesS3Access policy attached)
- ✅ IAM Access Advisor confirmed ses-email-sender was never used (safely deleted)

#### 1.2 Umami Setup ✅ COMPLETE

- [x] Separate `umami` database created (not sharing with main app database)
- [x] Umami service added to `docker-compose.yml`
- [x] Development nginx configuration updated with `/umami/` location block
- [x] Production nginx configuration verified (already had Umami config)
- [x] Automated setup script created: `scripts/setup-umami.sh`
  - Auto-detects environment (Docker container vs local)
  - Logs in with default credentials (admin/umami)
  - Creates or retrieves website configuration
  - Returns Website ID for .env configuration
- [x] `.env.development` configured:
  ```bash
  UMAMI_APP_SECRET=OjZyAUt+sAwGdPa1eEf8hIAZHOd3Rq0p
  UMAMI_WEBSITE_ID=98ec198c-b0b4-4189-96fa-3ed00745723b
  UMAMI_CORS_ORIGINS=https://localhost,http://localhost:3000
  ```

**Validation**: ✅ PASSED
- ✅ Umami running on port 3001 (external) / 3000 (internal)
- ✅ Accessible at `https://localhost/umami/` via nginx
- ✅ Setup script successfully creates/retrieves website configuration
- ✅ Default credentials work (admin/umami)

---

### Phase 2: Backend Repository Layer (TDD with Real Database) ✅ COMPLETE

**Pattern**: RED (write test) → GREEN (implement) → REFACTOR (clean up)

#### 2.1 Write Repository Tests (RED)
**File**: `backend/tests/repositories/testcontainers_blog_repository_tests.rs`

**Test cases to implement** (each follows RED → GREEN cycle):
1. `test_create_post` - Create draft post
2. `test_create_post_published` - Create published post (has published_at)
3. `test_get_post_by_id` - Fetch by UUID
4. `test_get_post_by_slug` - Fetch by slug
5. `test_list_posts_pagination` - Verify limit/offset
6. `test_list_posts_filter_by_status` - Only published/draft
7. `test_list_posts_filter_by_tag` - Posts with specific tag
8. `test_update_post` - Modify existing post
9. `test_delete_post` - Remove post from database
10. `test_search_posts_full_text` - PostgreSQL tsvector search
11. `test_get_all_tags_with_counts` - Tag aggregation
12. `test_slug_uniqueness_constraint` - Should error on duplicate

**TDD Cycle Example**:
```rust
// Step 1: RED - Write failing test
#[tokio::test]
async fn test_create_post() {
    let container = TestContainer::builder().build().await.unwrap();
    let repo = PostgresBlogRepository::new(container.pool.clone());

    let post_data = CreateBlogPost {
        title: "Test Post".into(),
        slug: "test-post".into(),
        content: "# Hello".into(),
        status: "draft".into(),
        tags: vec!["rust".into()],
        // ...
    };

    let post = repo.create_post(post_data).await.unwrap();

    assert_eq!(post.title, "Test Post");
    assert_eq!(post.status, "draft");
    assert!(post.published_at.is_none()); // Draft has no published_at
}

// Run: cargo test test_create_post -- --test-threads=4
// Result: ❌ Compilation error (repository doesn't exist)

// Step 2: GREEN - Implement minimum code to pass
// (See Repository Layer section above for implementation)

// Run: cargo test test_create_post -- --test-threads=4
// Result: ✅ Pass

// Step 3: REFACTOR - Clean up, optimize query, add comments
```

#### 2.2 Implement Repository (GREEN) ✅ COMPLETE

**Files created**:
- [x] `backend/tests/repositories/testcontainers_blog_repository_tests.rs` - 14 comprehensive tests
- [x] `backend/src/repositories/postgres/postgres_blog_repository.rs` - PostgreSQL implementation
- [x] `backend/tests/repositories/mod.rs` - Updated with blog repository module
- [x] `backend/src/repositories/postgres/mod.rs` - Updated with blog repository module
- [x] `backend/src/repositories/traits/blog_repository.rs` - Added `sqlx::FromRow` to `TagCount`

**Repository methods implemented**:
- [x] `create_post()` - Insert new blog post
- [x] `get_post_by_id()` - Find by UUID
- [x] `get_post_by_slug()` - Find by URL-friendly slug
- [x] `list_posts()` - Paginated list with status/tag filters (refactored with match pattern)
- [x] `update_post()` - Partial update with dynamic SET clause
- [x] `delete_post()` - Hard delete from database
- [x] `search_posts()` - Full-text search using PostgreSQL tsvector
- [x] `get_all_tags()` - Aggregate tags with counts, optionally filtered by status

**Validation**: ✅ PASSED
- ✅ All 14 repository tests passing
- ✅ Cargo clippy (no warnings)
- ✅ Cargo fmt (properly formatted)
- ✅ Cargo check (compiles successfully)
- ✅ SQLx cache updated (85 queries)

**Test coverage** (14 tests):
1. ✅ `test_create_post` - Create draft post
2. ✅ `test_create_post_published` - Create published post with timestamp
3. ✅ `test_get_post_by_id` - Fetch by UUID
4. ✅ `test_get_post_by_id_not_found` - Handle missing post
5. ✅ `test_get_post_by_slug` - Fetch by URL slug
6. ✅ `test_get_post_by_slug_not_found` - Handle missing slug
7. ✅ `test_list_posts_pagination` - Verify limit/offset
8. ✅ `test_list_posts_filter_by_status` - Filter by draft/published
9. ✅ `test_list_posts_filter_by_tag` - Filter posts by tag
10. ✅ `test_update_post` - Modify existing post
11. ✅ `test_delete_post` - Remove post from database
12. ✅ `test_search_posts_full_text` - PostgreSQL tsvector search
13. ✅ `test_get_all_tags_with_counts` - Tag aggregation with status filtering
14. ✅ `test_slug_uniqueness_constraint` - Enforce unique slug constraint

---

### Phase 3: Backend Service Layer (TDD with Mocks) ✅ COMPLETE

**Pattern**: Write unit test → Implement business logic → Refactor

#### 3.1 Service Implementation ✅ COMPLETE

**Files Created:**
1. `backend/src/services/blog/mod.rs` - BlogService with builder pattern
2. `backend/src/services/blog/utils.rs` - Helper functions (slugify, strip markdown, truncate)
3. `backend/src/services/blog/create.rs` - Create/publish operations + 6 tests
4. `backend/src/services/blog/read.rs` - Get/list/search operations + 6 tests
5. `backend/src/services/blog/update.rs` - Update operations + 4 tests
6. `backend/src/services/blog/delete.rs` - Delete operations + 4 tests
7. `backend/src/models/api/blog.rs` - API request/response models

**Test Coverage: 47/47 tests passing** ✅
- 6 create operation tests (slug generation, collision handling, excerpt generation, publish timestamp, validation)
- 6 read operation tests (get by ID/slug, list, search, tags)
- 4 update operation tests (preserve published_at, set on publish, validation, not found)
- 4 delete operation tests (remove images, without image, not found, graceful failure)
- 10 utility function tests (slugify, strip markdown, truncate text)
- 17 test builder tests (BlogPostBuilder validation)

**Business Logic Implemented:**
1. ✅ Auto-generates slug from title if not provided
2. ✅ Handles slug collisions (appends "-2", "-3", etc.)
3. ✅ Auto-generates excerpt from first 160 chars
4. ✅ Sets `published_at` timestamp when status is "published"
5. ✅ Preserves `published_at` for already-published posts on update
6. ✅ Cleans up S3 images on post deletion (via ImageStorage trait)
7. ✅ Validates title (not empty) and status (draft/published)

**Code Quality:**
- ✅ `cargo test --lib blog` → 47/47 passing (< 0.01s)
- ✅ `cargo clippy --lib` → no warnings
- ✅ `cargo fmt --all` → properly formatted
- ✅ Follows TDD methodology (RED → GREEN → REFACTOR)
- ✅ Dependency injection with trait abstractions
- ✅ Builder pattern for service construction
- ✅ Matches existing project service patterns

**Note:** Image upload validation tests (file size, filename sanitization, MIME type) deferred to Phase 4 (API layer) where multipart form data handling occurs

---

### Phase 4: Backend API Routes (TDD with HTTP Integration)

**Pattern**: Write API test → Implement route → Refactor

#### 4.1 Write API Tests (RED)
**File**: `backend/tests/api/testcontainers_blog_api_tests.rs`

**Test cases**:
1. `test_create_post_success` - Admin creates post → 200 OK
2. `test_create_post_requires_auth` - No token → 401
3. `test_create_post_requires_admin_role` - Regular user → 403
4. `test_get_post_by_slug_success` - Fetch published post → 200 OK
5. `test_get_post_by_slug_not_found` - Invalid slug → 404
6. `test_list_posts_pagination` - Verify page/limit params
7. `test_list_posts_filter_by_tag` - Tag filter works
8. `test_search_posts` - Full-text search returns results
9. `test_update_post_success` - Admin updates → 200 OK
10. `test_delete_post_success` - Admin deletes → 204 No Content
11. `test_upload_image_multipart` - Image upload → returns URLs
12. `test_rss_feed_returns_xml` - RSS feed → valid XML

#### 4.2 Implement API Routes (GREEN)
**File**: `backend/src/routes/blog.rs`

(See Routes section above for implementation)

**Validation**: `cargo test --test testcontainers_blog_api_tests -- --test-threads=4` → All pass

---

### Phase 5: Frontend Public Pages (SSR for SEO)

#### 5.1 Blog List Page
**File**: `frontend/app/pages/blog/index.vue`

- SSR data fetching with `useAsyncData`
- Post cards, pagination, tag filter
- SEO meta tags
- **HTTP Caching**: Add `Cache-Control: public, max-age=300` header

#### 5.2 Single Post Page
**File**: `frontend/app/pages/blog/[slug].vue`

- SSR rendering with full content
- Toast UI Viewer for markdown
- OG/Twitter meta tags
- Share buttons with Umami tracking
- "Updated" indicator (if edited >1 hour after publish)
- **HTTP Caching**: `Cache-Control: public, max-age=3600, must-revalidate`

#### 5.3 Components
**Files**:
- `components/blog/PostCard.vue`
- `components/blog/PostContent.vue`
- `components/blog/ShareButtons.vue`
- `components/blog/Pagination.vue`

**Frontend Tests** (Vitest):
```typescript
describe('PostCard', () => {
  it('displays post title and excerpt', () => {
    const post = { title: 'Test', excerpt: 'Excerpt', slug: 'test' }
    const wrapper = mount(PostCard, { props: { post } })
    expect(wrapper.text()).toContain('Test')
  })
})
```

**Validation**:
- Navigate to `/blog` → posts render
- View source → SSR HTML present
- Check `<head>` → OG tags present

---

### Phase 6: Frontend Admin Pages (Content Management)

#### 6.1 Markdown Editor with Auto-Save
**File**: `components/blog/MarkdownEditor.vue`

(See updated editor code above with debounced auto-save and mobile responsive)

#### 6.2 Image Upload Component
**File**: `components/blog/ImageUpload.vue`

- Drag-drop + file picker
- Preview thumbnail
- Upload to `/api/blog/admin/upload-image`
- Display featured URL

#### 6.3 Admin Pages
**Files**:
- `pages/admin/blog/index.vue` - List all posts (draft/published tabs)
- `pages/admin/blog/new.vue` - Create with auto-save
- `pages/admin/blog/[id]/edit.vue` - Edit with auto-save

**Frontend Tests**:
```typescript
describe('useBlogEditor auto-save', () => {
  it('saves draft 2s after typing stops', async () => {
    const { postForm, savePost } = useBlogEditor()

    postForm.value.title = 'Updated'

    await new Promise(resolve => setTimeout(resolve, 2100))

    expect(savePost).toHaveBeenCalled()
  })
})
```

**Validation**: Create, edit, delete posts via admin UI with auto-save working

---

### Phase 7: Umami Integration

#### 7.1 Add Tracking Script
**File**: `frontend/app/app.vue`

```vue
<script setup>
useHead({
  script: [{
    src: '/umami/script.js',
    'data-website-id': process.env.UMAMI_WEBSITE_ID,
    async: true,
    defer: true
  }]
})
</script>
```

#### 7.2 Track Custom Events
- Share buttons → `window.umami.track('blog-share', { platform, post_id, post_slug })`
- Post views → `window.umami.track('blog-view', { post_id, post_slug, tags })`

**Validation**:
- Check Umami dashboard → see page views
- Click share → see 'blog-share' event
- View post → see 'blog-view' event

---

### Phase 8: Polish & HTTP Caching

1. Add HTTP caching headers to backend routes:
   - Blog list: `Cache-Control: public, max-age=300`
   - Single post: `Cache-Control: public, max-age=3600, must-revalidate`
   - RSS feed: `Cache-Control: public, max-age=1800`

2. Performance testing:
   - Blog list loads <500ms
   - Single post loads <300ms

3. Mobile testing:
   - Editor switches to tab mode on mobile
   - All pages responsive

**Validation**: Lighthouse score >90, mobile UX tested

---

### Phase 9: Email Notifications (Optional - Post-MVP, Not Required for Shipping)

**Note**: This phase is optional and not required for MVP launch. Implement only if time allows.

**Goal:** Notify users via email when new blog posts are published

**Database:**
1. Add `blog_post_notifications` field to `user_preferences` table:
   ```sql
   ALTER TABLE user_preferences
   ADD COLUMN blog_post_notifications BOOLEAN DEFAULT true;
   ```

**Backend:**
1. Create domain event: `BlogPostPublishedEvent`
   - Fired when post status changes from 'draft' to 'published'
   - Contains: post_id, slug, title, excerpt, published_at

2. Create email notification handler:
   - Listens for `BlogPostPublishedEvent`
   - Queries users: `verified_at IS NOT NULL AND blog_post_notifications = true`
   - Respects email suppression list (AWS SES)
   - Sends notification emails asynchronously

3. Email template (HTML + plain text):
   - Subject: "New post: {post_title}"
   - Content: Post title, excerpt, "Read more" link
   - Footer: One-click unsubscribe link

4. Unsubscribe endpoint:
   - `GET /api/users/email/unsubscribe/blog-posts?token={signed_token}`
   - Sets `blog_post_notifications = false`
   - Shows confirmation page

**Frontend:**
1. User settings page:
   - Toggle: "Email me when new blog posts are published"
   - Updates `blog_post_notifications` preference

2. Unsubscribe confirmation page:
   - `/unsubscribe/success` - Confirms email notifications disabled
   - Link to re-enable in settings

**Email Details:**
- **Service:** AWS SES (existing infrastructure)
- **Delivery:** Asynchronous (doesn't block publish operation)
- **Rate limiting:** Batch send to avoid SES limits
- **Tracking:** Umami event on email click (optional)

**Compliance:**
- ✅ Opt-out available (user preference toggle)
- ✅ One-click unsubscribe in every email
- ✅ Signed unsubscribe tokens (prevent abuse)
- ✅ Respects email suppression list
- ✅ GDPR/CCPA compliant

**Validation:**
- Publish blog post triggers email to subscribed users
- Unsubscribe link sets preference to false
- Re-publishing same post doesn't send duplicate emails
- User can toggle preference in settings
- Emails respect suppression list

---

## Testing Strategy

### Backend Tests
- **Unit tests:** Service layer with mock repositories (MockBlogRepository, MockImageStorage)
- **Integration tests:** Full API tests with testcontainers PostgreSQL
- **Image storage tests:** MockImageStorage for service unit tests, S3ImageStorage integration tests (optional)

### Frontend Tests
- **Component tests:** Vitest for isolated component behavior
- **E2E tests:** Playwright for critical flows (create post, publish, view)

### Manual Testing Checklist
- [ ] Create draft post
- [ ] Upload featured image (verify S3 upload)
- [ ] Publish post
- [ ] View post as public user
- [ ] Verify OG tags in view-source
- [ ] Test social share buttons (click through to Twitter/FB)
- [ ] Search for post
- [ ] Filter by tag
- [ ] Edit post
- [ ] Delete post
- [ ] Verify RSS feed

---

## Environment Variables

### `.env.development`
```bash
# S3 Configuration
AWS_REGION=us-east-1
AWS_S3_BUCKET_BLOG_IMAGES=kennwilliamson-blog-images
AWS_ACCESS_KEY_ID=<your-key>
AWS_SECRET_ACCESS_KEY=<your-secret>

# Optional: CloudFront distribution
CLOUDFRONT_DOMAIN=d1234.cloudfront.net

# Umami Analytics
UMAMI_APP_SECRET=<random-32-char-string>  # For session encryption
UMAMI_WEBSITE_ID=<get-from-umami-dashboard>  # After creating website in Umami
```

### Deployment Checklist
- [ ] Create S3 bucket in production AWS account
- [ ] Set S3 bucket policy for public read
- [ ] Create IAM user with S3 write permissions
- [ ] Add AWS credentials to production environment
- [ ] Optional: Set up CloudFront distribution
- [ ] Deploy Umami service (Docker)
- [ ] Complete Umami initial setup (create website, get tracking ID)
- [ ] Add Umami environment variables to production
- [ ] Configure Nginx proxy for `/umami` path
- [ ] Run database migrations

---

## Open Questions / Decisions Needed

1. **Image storage:** Start with S3, add CloudFront later?
   - **Decision:** Start with S3 only, add CloudFront if performance becomes issue

2. **Analytics:** Custom vs Umami vs Google Analytics?
   - **Decision:** ✅ Umami - Self-hosted, privacy-focused, comprehensive analytics without reinventing the wheel

3. **Markdown editor:** Use existing library or build custom?
   - **Decision:** ✅ Toast UI Editor - Split-view with live preview, good UX, active maintenance
   - **Rendering:** ✅ Toast UI Viewer - Consistent rendering with editor

4. **Tag management:** Free-form or predefined tags?
   - **Decision:** Free-form for flexibility, can add suggestions later

5. **Search:** PostgreSQL full-text or external (Algolia/Meilisearch)?
   - **Decision:** PostgreSQL full-text for MVP (free, no external deps)

---

## Success Criteria

### Backend Testing (TDD Compliance)

**All tests must pass before proceeding to next phase:**

- [ ] **Repository tests pass**: `cargo test --test testcontainers_blog_repository_tests -- --test-threads=4`
  - All 12 repository test cases green
  - Tests use real PostgreSQL via TestContainer
  - SQLx cache updated with `./scripts/prepare-sqlx.sh`

- [ ] **Service tests pass**: `cargo test --lib blog_service`
  - All 9 service test cases green
  - Tests use MockRepository (fast unit tests)
  - Business logic verified (slug collision, excerpt generation, S3 cleanup, etc.)

- [ ] **API tests pass**: `cargo test --test testcontainers_blog_api_tests -- --test-threads=4`
  - All 12 API test cases green
  - Tests use TestContext (full HTTP integration)
  - Auth/authorization verified (401, 403 responses)

- [ ] **Overall backend coverage**: Maintain ~620+ tests project-wide

### Frontend Testing

- [ ] **Component tests pass**: `npm test -- blog`
  - PostCard, PostContent, ShareButtons, Pagination tests green
  - Auto-save debounce logic tested

- [ ] **SSR verification**: View source shows server-rendered HTML
- [ ] **OG tags present**: Check `<head>` for OpenGraph/Twitter meta tags

### Functional Requirements

- [ ] **Admin CRUD operations**:
  - [ ] Create blog post with markdown editor (Toast UI with live preview and mobile tab mode)
  - [ ] Upload featured images (stored in S3 with security validation)
  - [ ] Edit post with auto-save (debounced 2s after typing stops)
  - [ ] Delete post (also deletes S3 images immediately)
  - [ ] Publish/unpublish (sets `published_at` timestamp)

- [ ] **Public display**:
  - [ ] Posts display on `/blog` (reverse chronological, paginated)
  - [ ] Single post page `/blog/[slug]` renders markdown with Toast UI Viewer
  - [ ] "Updated" indicator shows when edited >1 hour after publish
  - [ ] Search returns relevant results (PostgreSQL full-text search)
  - [ ] Tag filtering works (click tag → filtered list)

- [ ] **Social sharing & SEO**:
  - [ ] OG images and meta tags present (1200x630px featured images)
  - [ ] Share buttons work (Twitter, Facebook, LinkedIn)
  - [ ] RSS feed validates (uses `rss` crate, not manual XML)
  - [ ] SSR for SEO (server-rendered HTML)

- [ ] **Analytics (Umami)**:
  - [ ] Umami dashboard accessible at `/umami`
  - [ ] Tracking script loads correctly on all pages
  - [ ] Page views tracked automatically
  - [ ] Custom events tracked (blog-share, blog-view with metadata)
  - [ ] Admin can access analytics dashboard

### Security Requirements

- [ ] **Image upload security**:
  - [ ] File size validated (<5MB)
  - [ ] Filename sanitized (no path traversal)
  - [ ] MIME type verified (image crate loads successfully)
  - [ ] UUID-based storage (prevents collisions/injection)

- [ ] **Authentication & Authorization**:
  - [ ] CSRF protection verified for admin mutations
  - [ ] Admin role required for create/update/delete operations
  - [ ] Unauthorized requests return 401
  - [ ] Insufficient permissions return 403

### Performance Requirements

- [ ] **Page load times**:
  - [ ] Blog list loads <500ms
  - [ ] Single post loads <300ms
  - [ ] RSS feed responds <200ms

- [ ] **HTTP Caching**:
  - [ ] Blog list: `Cache-Control: public, max-age=300`
  - [ ] Single post: `Cache-Control: public, max-age=3600, must-revalidate`
  - [ ] RSS feed: `Cache-Control: public, max-age=1800`

- [ ] **Lighthouse score >90** (performance, SEO, accessibility)

### Design & UX Requirements

- [ ] **Mobile responsive**:
  - [ ] Editor switches to tab mode on <768px screens
  - [ ] All pages responsive (cards, pagination, images)
  - [ ] Touch-friendly tap targets

- [ ] **Auto-save works**:
  - [ ] Drafts saved 2 seconds after user stops typing
  - [ ] Visual indicator of save status (optional but nice)

### Infrastructure Requirements

- [ ] **Database migration**:
  - [ ] `blog_posts` table created with all indexes
  - [ ] `update_updated_at_column()` trigger function exists
  - [ ] Migration runs successfully with `./scripts/setup-db.sh`

- [x] **S3 configuration**: ✅ COMPLETE
  - [x] Bucket `kennwilliamson-blog-images` created
  - [x] Public read access configured
  - [x] CORS configuration for web uploads
  - [x] IAM permissions set for backend (using existing cli-admin credentials)
  - [x] AWS credentials in `.env.development` (not committed - file added to .gitignore)

- [ ] **Umami configuration**:
  - [ ] Service running on port 3001 (no conflict with Nuxt)
  - [ ] Nginx `/umami` proxy configured
  - [ ] Database schema created automatically
  - [ ] Website configured, tracking ID obtained

### TDD Process Compliance

- [ ] **Tests written before code**: Each feature implemented with RED → GREEN → REFACTOR cycle
- [ ] **No skipped tests**: All tests passing, no `.skip()` or `#[ignore]`
- [ ] **Test coverage maintained**: Project-wide test count increases appropriately
- [ ] **Integration tests use `--test-threads=4`**: Prevent Docker resource exhaustion
