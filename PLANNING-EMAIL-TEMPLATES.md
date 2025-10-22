# Phase 1: Email Template System

## Executive Summary

Implement a flexible HTML email template system with reusable components and send admin notifications when users request access.

**Estimated Effort**: 6-8 hours

**Goals**:
- Support HTML emails with professional branding
- Decouple email content from delivery mechanism
- Reusable email components (headers, footers, buttons)
- Notify all admins when access requests are created
- Maintain backward compatibility with existing emails

**Dependencies**: None (AWS SES SDK already present)

---

## Architecture Overview

### Core Components

**EmailTemplate Trait**:
- Abstraction for email content generation
- Implementations provide HTML and plain text versions
- Returns subject line and rendered bodies

**EmailService Trait**:
- Abstraction for email delivery
- Current implementation: SesEmailService (AWS SES SDK)
- Future implementations: SmtpEmailService, MockEmailService

**Template System** (Askama):
- Compile-time HTML template engine
- Automatic HTML escaping for security
- Jinja2-like syntax (familiar to many developers)
- Template inheritance and includes for component reuse
- Type-safe: Template variables match struct fields at compile time

**SesEmailService**:
- Uses AWS SES SDK directly (no lettre dependency)
- Multipart MIME support (HTML + plain text)
- Suppression list checking (already implemented)
- Respects email preferences

### Design Decisions

**Use AWS SES SDK Directly (No lettre)**:
- **Rationale**: YAGNI (You Aren't Gonna Need It)
- EmailService trait already provides abstraction for swapping providers
- Domain events (Phase 2) further decouple email from business logic
- Adding lettre later is straightforward if multi-provider support needed
- Estimated refactoring cost if needed: 1-2 hours (isolated to EmailService impl)
- **Trade-off**: Tighter coupling to AWS types, but simpler implementation now

**Askama Template Engine**:
- **Rationale**: Compile-time safety and automatic HTML escaping outweigh minimal complexity
- **Key Benefits**:
  - XSS protection by default (auto-escapes `<`, `>`, `&`, `"`, `'`)
  - Compile-time template validation (errors = build failures)
  - Type-safe templates (mismatched variables caught at compile time)
  - Designer-friendly (HTML files, not Rust strings)
  - Familiar syntax (Jinja2-like, widely known)
- **Trade-offs**:
  - Additional dependency (askama crate)
  - Templates require recompilation to change (acceptable - templates are code)
  - Small learning curve (minimal if familiar with Jinja2/Django/Twig)
- **Alternative Considered**: String-based templates rejected due to high XSS risk and poor maintainability
- **Documentation**: Use Context7 for `/lettre/lettre` and web search "askama rust" for examples
- **Performance**: Fastest template engine (~330µs vs 857µs for Tera, 3.6ms for Handlebars)

**Fire-and-Forget Email Sending**:
- **Rationale**: Email failures shouldn't block user operations
- Log errors but don't propagate to caller
- Access request creation succeeds even if notification fails
- **Trade-off**: Admins might miss notifications if email misconfigured

### Architectural Layers: Askama, AWS SES, and Lettre

**Three Distinct Concerns** (complementary, not overlapping):

1. **Content Generation** (Askama):
   - Converts data → HTML/text strings
   - Handles template rendering and escaping
   - Layer 1: "What to say"

2. **Message Construction** (AWS SES types now, optionally Lettre later):
   - Builds MIME multipart structure
   - Handles headers, recipients, encoding
   - Layer 2: "How to format"

3. **Transport/Delivery** (AWS SES SDK now, optionally Lettre SMTP later):
   - Actually sends the email
   - Handles SMTP or API calls
   - Layer 3: "How to deliver"

**Why This Matters**:
- Askama and Lettre are **complementary** - Askama generates content, Lettre could build/send
- Current approach: Askama (layer 1) → AWS SES types (layer 2) → AWS SES SDK (layer 3)
- Future option: Askama (layer 1) → Lettre (layers 2+3) → SMTP/SendGrid/etc
- EmailService trait abstracts layers 2+3, making migration seamless
- Template layer (1) completely independent from transport layers (2+3)

**Migration Flexibility**:
- Swap transport without touching templates
- Swap templates without touching transport
- Each layer independently testable and replaceable

---

## Task Breakdown

### Task 1: Design EmailTemplate Trait & Email Struct (1 hour)

**Objective**: Define core abstractions for email generation.

**EmailTemplate Trait**:
- `render_html(&self) -> Result<String>` - Generate HTML version (can fail if template broken)
- `render_plain_text(&self) -> String` - Generate text version
- `subject(&self) -> String` - Email subject line
- Trait bounds: `Send + Sync` for async usage

**Email Struct**:
- Fields: `to`, `subject`, `html_body`, `text_body`, `reply_to` (optional)
- Consider builder pattern vs direct construction
- Document multi-recipient behavior clearly

**Key Design Points**:
- EmailTemplate abstracts content generation (layer 1)
- Email struct is transport-agnostic intermediate representation (layer 2)
- EmailService handles delivery (layer 3)
- Clean separation enables swapping any layer independently

### Task 2: Create Askama Template Components (1 hour)

**Directory**: `backend/templates/emails/components/`

**Component Templates to Create**:
- `header.html` - Branded header with logo/site name
- `footer.html` - Standard footer (copyright, unsubscribe placeholder)
- `button.html` - Styled CTA button (accepts text, url, color via includes)
- `styles.html` - Shared CSS for all email templates
- `base.html` - Base layout with template inheritance

**Askama Syntax for Components**:
- Use `{% include "emails/components/header.html" %}` to embed components
- Use `{% extends "emails/base.html" %}` for template inheritance
- Use `{% block content %}` for overridable sections
- Variables automatically HTML-escaped unless `| safe` filter applied

**Email CSS Considerations**:
- Use inline styles (email clients strip `<style>` tags)
- Table-based layouts for Outlook compatibility
- Responsive design with media queries
- Steampunk theme colors (consistent with site)
- Test in major clients: Gmail, Outlook, Apple Mail

**Documentation References**:
- Askama template syntax: askama.readthedocs.io/en/latest/template_syntax.html
- Askama includes/inheritance: Search for "askama template inheritance"
- Email CSS best practices: Web search "html email css inline styles"

### Task 3: Implement Email Templates (2 hours)

**Rust Structs** (`backend/src/services/email/templates/`):
- `verification_email.rs` - Struct with `#[derive(Template)]` attribute
- `password_reset_email.rs` - Struct with `#[derive(Template)]` attribute
- `access_request_notification.rs` - Struct with `#[derive(Template)]` attribute

**Template Files** (`backend/templates/emails/`):
- `verification.html` - Email verification template
- `password_reset.html` - Password reset template
- `access_request.html` - Access request notification template

**VerificationEmailTemplate**:
- Struct fields: `to_name`, `verification_url`
- Template path: `#[template(path = "emails/verification.html")]`
- Subject: "Verify Your Email Address"
- Content: Verification link, 24-hour expiration notice, personalized greeting

**PasswordResetEmailTemplate**:
- Struct fields: `to_name`, `reset_url`
- Template path: `#[template(path = "emails/password_reset.html")]`
- Subject: "Reset Your Password"
- Content: Reset link, 1-hour expiration, security disclaimer

**AccessRequestNotificationTemplate**:
- Struct fields: `user_display_name`, `request_message`, `requested_role`, `admin_panel_url`
- Template path: `#[template(path = "emails/access_request.html")]`
- Subject: "New Access Request from {user_display_name}"
- **Security**: `request_message` automatically HTML-escaped by Askama (no manual escaping needed)
- Content: User info, request message, admin panel link

**Implementation Pattern**:
- Each struct derives `Template` from askama
- Implements `EmailTemplate` trait (delegates `render_html()` to Askama's `render()`)
- Plain text version uses format! strings (simple, no XSS risk in plain text)

### Task 4: Update EmailService Trait (0.5 hours)

**File**: `backend/src/services/email/mod.rs`

**Add Core Method**:
- `send_email(&self, email: Email) -> Result<()>` - Accepts generic Email struct

**Optional Convenience Methods**:
- Consider adding `send_verification_email()`, `send_password_reset_email()` helpers
- Decision: Convenience methods vs single generic method
- Trade-off: Ergonomics vs trait complexity

**Backward Compatibility Strategy**:
- Keep existing methods during migration
- Migrate callers incrementally
- Remove deprecated methods after full migration
- Document deprecation timeline

### Task 5: Update SesEmailService Implementation (1.5 hours)

**File**: `backend/src/services/email/ses_email_service.rs`

**Implementation Steps**:
1. Check suppression list for each recipient (existing functionality)
2. Build multipart MIME message using AWS SES SDK types
3. Send emails (individual vs bulk - decide during implementation)
4. Handle errors with appropriate logging

**AWS SES SDK Types Needed**:
- `Content` - For subject and body content
- `Body` - Wraps HTML and text parts
- `Message` - Combines subject and body
- `Destination` - Recipient addresses
- `EmailContent` - Wraps message for sending

**Key Architectural Decisions**:
- **Individual vs Bulk Sending**: Trade-off between privacy/cost and simplicity
  - Individual: Better privacy, more API calls, higher cost
  - Bulk: Single API call, recipients visible to each other, lower cost
  - Recommendation: Individual for admin notifications (privacy)
- **Partial Failure Handling**: What if some recipients fail?
  - Log each failure individually
  - Return success if any email sent
  - Document behavior clearly

**Documentation Reference**:
- AWS SES SDK docs: docs.rs/aws-sdk-sesv2
- Existing SesEmailService implementation for suppression list patterns

### Task 6: Add AdminRepository Method (0.5 hours)

**Files**:
- `backend/src/repositories/traits/admin_repository.rs` - Add trait method
- `backend/src/repositories/postgres/postgres_admin_repository.rs` - Implement query
- `backend/src/repositories/mock/mock_admin_repository.rs` - Mock implementation

**Method Signature**:
- `get_admin_emails(&self) -> Result<Vec<String>>`
- Returns list of admin email addresses for notification purposes

**Query Requirements**:
- Join `user_credentials`, `user_roles`, and `roles` tables
- Filter for 'admin' role
- Only include verified emails (email_verified = true)
- Handle NULL emails gracefully

**Open Decisions**:
- Filter by user status (active vs suspended)? Probably yes
- Cache admin emails? Probably unnecessary (infrequent operation)
- Empty result handling? Log warning, don't fail

**Testing Approach**:
- Add mock implementation for testing
- Test with zero admins, one admin, multiple admins
- Test filtering (unverified emails excluded)

### Task 7: Update AccessRequestModerationService (1 hour)

**File**: `backend/src/services/admin/access_request_moderation/mod.rs`

**Refactor to Builder Pattern**:
- Add new dependencies: `AdminRepository`, `EmailService`, `frontend_url`
- Make email dependencies optional (service works without them)
- Use builder pattern for flexible construction

**Builder Methods Needed**:
- `with_access_request_repository()` - Required
- `with_admin_repository()` - Optional (needed for email notifications)
- `with_email_service()` - Optional (needed for email notifications)
- `with_frontend_url()` - Optional (needed for admin panel links in emails)
- `build()` - Validate and construct service

**Update `create_request()` Method**:
1. Create access request in database (existing logic)
2. If email dependencies configured:
   - Fetch admin emails via AdminRepository
   - Build AccessRequestNotificationTemplate (from Task 3)
   - Render template to HTML/text
   - Send email via EmailService
   - Log errors but don't propagate (fire-and-forget)

**Method Signature Changes**:
- Add parameters: `user_email`, `user_display_name` (needed for email template)
- Keep existing parameters: `user_id`, `message`, `requested_role`
- Return type unchanged: `Result<()>`

**Error Handling Strategy**:
- Database failures: Propagate error (critical)
- Email failures: Log but don't propagate (non-critical)
- Missing dependencies: Skip email sending, log info message

### Task 8: Wire Up Container (0.5 hours)

**File**: `backend/src/container.rs`

**Update Service Construction**:
- Use AccessRequestModerationService builder pattern
- Inject all dependencies via builder methods
- Pass optional dependencies conditionally

**Configuration Management**:
- Add `FRONTEND_URL` environment variable (e.g., `https://kennwilliamson.org`)
- Validate configuration at startup
- Log warnings for missing optional dependencies

**Validation Logic**:
- If EmailService configured but FRONTEND_URL missing: Warn (incomplete email config)
- If FRONTEND_URL set but EmailService missing: Info (emails disabled)
- If both missing: Info (email notifications disabled)

**Environment Documentation**:
- Update `.env.example` with FRONTEND_URL
- Document that email notifications require both EmailService and FRONTEND_URL
- Document that missing config gracefully degrades (no emails sent)

### Task 9: Testing (1 hour)

**Test Files**:
- `backend/tests/email_templates/template_tests.rs` - Template rendering
- `backend/tests/email_templates/components_tests.rs` - Component functions
- `backend/src/services/admin/access_request_moderation/mod.rs` - Service tests

**Test Scenarios**:
- EmailTemplate implementations render valid HTML
- Plain text fallback has correct content
- HTML escaping prevents XSS (test with `<script>` tags)
- AccessRequestModerationService sends emails to all admins
- Email failures don't break request creation
- MockEmailService captures emails for assertions
- Suppression list respected (no emails to suppressed addresses)

**Testing Strategy**:
- Use MockEmailService to capture sent emails
- Assert email content matches expected template output
- Test with and without email_service configured
- Test error scenarios (email service failures)

---

## Simplified TDD Iterations

### Iteration 1: EmailTemplate Trait & Email Struct
**Red**: Write tests for trait and struct
**Green**: Implement trait and struct
**Refactor**: Add documentation, ensure Send + Sync

### Iteration 2: Email Components
**Red**: Write tests for header, footer, button, base_layout
**Green**: Implement component functions
**Refactor**: Extract CSS constants, improve HTML structure

### Iteration 3: Concrete Templates
**Red**: Write tests for verification, password reset, access request templates
**Green**: Implement EmailTemplate for each
**Refactor**: Extract common patterns, ensure consistent styling

### Iteration 4: EmailService Updates
**Red**: Write tests for send_email() method
**Green**: Implement in SesEmailService and MockEmailService
**Refactor**: Improve error handling, add logging

### Iteration 5: Service Integration
**Red**: Write tests for AccessRequestModerationService email notifications
**Green**: Update service to send emails after creating requests
**Refactor**: Extract email notification logic to private method

### Iteration 6: End-to-End Validation
**Red**: Write integration test for full access request flow
**Green**: Wire up container, verify emails sent
**Refactor**: Optimize, clean up, verify all tests pass

---

## File Structure

```
backend/
├── src/
│   └── services/
│       └── email/
│           ├── mod.rs                              # EmailService trait, Email struct
│           ├── ses_email_service.rs                # AWS SES implementation
│           ├── mock_email_service.rs               # Mock for testing
│           └── templates/
│               ├── mod.rs                          # EmailTemplate trait, re-exports
│               ├── verification_email.rs           # Verification email struct
│               ├── password_reset_email.rs         # Password reset struct
│               └── access_request_notification.rs  # Access request struct
├── templates/                                      # Askama templates
│   └── emails/
│       ├── base.html                               # Base layout (template inheritance)
│       ├── verification.html                       # Verification email HTML
│       ├── password_reset.html                     # Password reset email HTML
│       ├── access_request.html                     # Access request email HTML
│       └── components/
│           ├── styles.html                         # Shared CSS
│           ├── header.html                         # Email header
│           ├── footer.html                         # Email footer
│           └── button.html                         # CTA button
└── tests/
    └── email_templates/
        ├── template_tests.rs                       # Template rendering tests
        └── xss_tests.rs                            # XSS prevention tests
```

**Key Points**:
- Rust structs in `src/services/email/templates/` (derive Template)
- HTML templates in `templates/emails/` (Askama convention)
- Components in `templates/emails/components/` (reusable via includes)
- Tests focus on XSS prevention and rendering correctness

---

## Architecture Patterns

### Trait-Based Abstraction
- **EmailService trait**: Allows swapping email providers (SES, SMTP, Mailgun)
- **EmailTemplate trait**: Consistent interface for all email types
- **Repository traits**: Decouple service from data access

### Builder Pattern
- **AccessRequestModerationService**: Use builder for flexible construction
- **Email struct**: Could add builder (decide during implementation)

### Template Pattern
- **EmailTemplate trait**: Define structure, let implementations customize
- **Component composition**: Reusable HTML/text builders

### Error Handling Strategy
- **Non-critical failures**: Log errors, don't propagate (email sending)
- **Critical failures**: Propagate errors (database operations)
- **User experience**: Never block user operations due to email failures

---

## Technical Considerations

### HTML Email Best Practices
- **Inline CSS**: Email clients strip `<style>` tags
- **Table layouts**: Outlook requires table-based layouts
- **Alt text**: Provide alt text for images (accessibility)
- **Plain text fallback**: Always provide plain text version
- **Unsubscribe link**: Good practice for transactional emails
- **Mobile responsive**: Use media queries for mobile devices
- **No JavaScript**: Not supported in email clients

### XSS Prevention
- **Automatic escaping**: Askama auto-escapes all variables in .html templates
- **Characters escaped**: `<`, `>`, `&`, `"`, `'` (OWASP recommendations)
- **Opt-out when needed**: Use `| safe` filter for trusted HTML only
- **Test with malicious input**: Test templates with `<script>`, `<img>`, event handlers
- **Plain text is safe**: Text version doesn't need escaping (not rendered as HTML)
- **Compile-time safety**: Template errors = build failures (catch before production)

### AWS SES Considerations
- **Multipart MIME**: Send both HTML and text in single email
- **Suppression list**: Already implemented, continue checking
- **Rate limiting**: AWS SES has sending limits (monitor usage)
- **Configuration sets**: Can add later for delivery metrics
- **Tags**: Can add later for categorization

### Email Client Compatibility
- **Test targets**: Gmail, Outlook, Apple Mail, Yahoo Mail
- **CSS support**: Limited, use inline styles
- **Image blocking**: Design emails to work without images
- **Font fallbacks**: Use web-safe fonts with fallbacks

### Admin Email Query
- **Filter verified emails only**: Only send to verified admin emails
- **Consider user status**: Should we exclude suspended/inactive users?
- **Caching**: Probably not necessary (infrequent operation)
- **Empty results**: Handle gracefully (log warning)

---

## Success Criteria

- ✅ All emails send as HTML with plain text fallback
- ✅ Email templates are reusable and DRY
- ✅ Admins receive notifications when access requests are created
- ✅ Existing verification and password reset emails migrated
- ✅ All tests pass (~10 new tests)
- ✅ No breaking changes to EmailService API from consumer perspective
- ✅ HTML rendering works in major email clients
- ✅ Email sending failures don't block user operations

---

## Migration Path

### Step 1: Add Infrastructure (Non-Breaking)
1. Add `html-escape` dependency to `Cargo.toml`
2. Create `templates/` module with EmailTemplate trait
3. Implement email components (header, footer, button, base_layout)
4. No existing code changes yet

### Step 2: Implement New Templates
1. Create VerificationEmailTemplate
2. Create PasswordResetEmailTemplate
3. Create AccessRequestNotificationTemplate
4. Write tests for each template

### Step 3: Update EmailService Trait (Backward Compatible)
1. Add `send_email(Email)` method to trait
2. Implement in SesEmailService
3. Implement in MockEmailService
4. Keep existing methods (don't remove yet)

### Step 4: Migrate Existing Emails
1. Update AuthService to use new verification template
2. Update AuthService to use new password reset template
3. Test existing flows still work
4. Remove old email body generation code from SesEmailService

### Step 5: Add New Feature
1. Add `get_admin_emails()` to AdminRepository
2. Update AccessRequestModerationService with builder pattern
3. Add email notification logic to `create_request()`
4. Wire up in container

### Step 6: Clean Up
1. Remove old EmailService methods (if no longer used)
2. Update tests to use new patterns
3. Run full test suite
4. Manual testing in dev environment

---

## Dependencies to Add

```toml
[dependencies]
askama = "0.14"  # Compile-time HTML template engine with auto-escaping
```

**Dependency Rationale**:
- **askama**: Provides compile-time template checking and automatic HTML escaping
- **No lettre**: Using AWS SES SDK directly (EmailService trait enables future migration)
- **No html-escape**: askama handles escaping automatically

**Alternative Considered**:
- String-based templates: Rejected due to XSS risk and maintainability concerns
- Tera/Handlebars: Rejected due to runtime overhead and lack of compile-time checking

**Migration Path if Needed**:
- Askama → Tera: Change derive macro, templates stay mostly the same (both Jinja2-like)
- AWS SES → Lettre: Swap EmailService implementation, no template changes needed
- Templates are isolated from transport layer via EmailService trait

---

## Open Questions (Decide During Implementation)

### Email Sending Strategy
- **Individual vs. Bulk**: Send individual emails to each admin or single email with all in To field?
  - Individual: Better privacy, more API calls, higher cost
  - Bulk: Single API call, less privacy, recipients see each other
  - **Recommendation**: Individual emails for admin notifications (privacy)

### Email Builder Pattern
- **Multiple recipients**: Should `to()` accumulate or `multiple_recipients()` replace?
  - Option 1: `add_recipient()` accumulates, `with_recipients()` replaces
  - Option 2: Multiple `to()` calls accumulate
  - **Decision during implementation**: Choose based on API ergonomics

### Admin Email Filtering
- **User status**: Filter by active/suspended status?
  - Probably yes - only send to active, verified admins
  - Query needs join to users table for status

### Error Notification Strategy
- **All emails fail**: What happens if no admin emails found or all fail?
  - Log warning (already planned)
  - In-app notification fallback? (Future enhancement)
  - Health check should verify email service configured

### Frontend URL Construction
- **Environment variable**: `FRONTEND_URL=https://kennwilliamson.org`
  - Required if email_service configured
  - Log warning at startup if missing
  - Document in `.env.example`

### Email Footer Content
- **Unsubscribe link**: Include? (Good practice but not required for transactional)
- **Contact info**: Support email? Social media links?
- **Steampunk styling**: How much branding in footer?
- **Decision**: Design during component implementation

### CSS Styling Details
- **Color scheme**: Match site steampunk theme (browns, coppers, dark backgrounds)
- **Button styles**: Brass/copper metallic look?
- **Typography**: Serif fonts for steampunk aesthetic?
- **Decision**: Implement during component creation, can iterate

---

## Future Enhancements (Post-Phase 1)

### Email Provider Flexibility (Lettre Integration)
- Add Lettre for multi-provider support if needed
- Implement LettreEmailService (uses Lettre's message builder + transport)
- Swap in container - no changes to templates or services
- Benefits: SMTP support, SendGrid/Mailgun integration, file transport for testing
- Effort: ~1-2 hours (isolated to EmailService implementation)

### Email Analytics
- Track email opens (tracking pixel)
- Track link clicks (redirect links)
- Store metrics in database
- Dashboard for email performance

### Email Preferences
- User preferences for email types
- Frequency controls (daily digest vs. immediate)
- Unsubscribe management
- Preference center UI

### Additional Email Types
- Welcome email for new users
- Email verification reminders
- Password change confirmation
- Account activity notifications
- Timer sharing notifications

### Email Testing Tools
- Preview endpoint (render templates without sending)
- Test email sender (send to specific address)
- Email client screenshot testing
- Automated HTML validation

### Internationalization
- Multi-language email templates
- User language preference
- Translation management

---

## Notes for Implementer

### Before You Start
- Review Askama documentation: askama.readthedocs.io
- Review AWS SES SDK documentation for multipart MIME
- Understand email client CSS limitations (inline styles, table layouts)
- Study existing SesEmailService implementation for suppression list patterns
- Plan test data including XSS attack vectors (`<script>`, `<img onerror>`, etc.)

### During Implementation
- Test HTML in actual email clients early (don't wait until end)
- Use MockEmailService for fast iteration
- Log verbosely during development (can reduce later)
- Create test fixtures for common scenarios

### Testing Approach
- Write template tests first (easiest to test)
- Test components in isolation
- Integration tests for full email flow
- Manual testing with real email addresses
- Test suppression list integration

### Common Pitfalls
- Using `| safe` filter unnecessarily (defeats XSS protection)
- Using block-level CSS (not supported in many email clients)
- Assuming images will load (design for image blocking)
- Not testing plain text version
- Breaking existing email functionality during migration
- Forgetting to create `templates/` directory (Askama won't compile)
- Mismatching template variables and struct fields (will cause compile error)

---

**Document Version**: 2.0
**Created**: 2025-01-21
**Updated**: 2025-01-22 (Added Askama template engine decision)
**Status**: Ready for Implementation
**Estimated Effort**: 6-8 hours
**Dependencies**: askama = "0.14" (AWS SES SDK already present)
**Blocks**: Phase 2 (Domain Events) depends on EmailService trait from this phase

**Key Decisions Made**:
- ✅ Use Askama for templates (compile-time safety, auto-escaping)
- ✅ Use AWS SES SDK directly (no lettre for now)
- ✅ EmailService trait provides migration path to Lettre if needed
- ✅ Three-layer architecture: Content (Askama) → Message (AWS types) → Transport (SES SDK)
