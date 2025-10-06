# Privacy Policy & Terms of Service Compliance Gaps

**Generated:** October 4, 2025
**Status:** Pre-production compliance review

## Executive Summary

The Privacy Policy and Terms of Service make explicit promises about features that are **not currently implemented**. This creates legal liability under GDPR, CCPA, and contract law.

**Critical Issues:** 2 features must be implemented before production deployment (1 completed)
**Important Issues:** 3 features should be implemented for compliance and user trust
**Total Implementation Time:** Estimated 2-3 weeks for remaining critical + important features

---

## Critical Gaps (Must Fix Before Production)

### 1. Account Deletion Functionality ✅ COMPLETED

**Legal Liability:** HIGH
**Estimated Effort:** 2-3 days

**Where Claimed:**
- Privacy Policy: "You can delete your account at any time through your profile settings. This will remove your account data within 30 days."
- Terms of Service: "You can delete your account at any time through your profile settings."

**Current Status:** ✅ IMPLEMENTED
- ✅ Delete account API endpoint: `DELETE /backend/protected/auth/delete-account`
- ✅ Delete button in profile settings with confirmation modal
- ✅ Soft delete mechanism with `deleted_at` timestamp
- ✅ 30-day grace period with permanent deletion
- ✅ Confirmation email with cancellation link
- ✅ Proper cascade handling for user content

**Compliance Risk:**
- **GDPR Article 17** (Right to Erasure) violation - could face fines up to €20M or 4% of global revenue
- **CCPA Section 1798.105** violation - could face fines up to $7,500 per intentional violation
- **Contract law** - false promise in legally binding terms

**Implementation Required:**
1. Backend API endpoint: `DELETE /backend/protected/auth/delete-account`
2. Database migration: Add `deleted_at` timestamp column to `users` table
3. Service method: `AuthService::delete_account()` with 30-day soft delete
4. Frontend UI: Delete account button/modal in profile settings
5. Scheduled job: Permanent deletion after 30-day grace period
6. Handle cascade: Decide on user content (timers, phrases) - delete or anonymize
7. Confirmation email: Notify user of deletion with cancellation link

**Files to Modify:**
- `backend/src/routes/auth.rs` - Add delete endpoint
- `backend/src/services/auth/auth_service/account.rs` - New file for account management
- `backend/migrations/` - New migration for soft delete
- `frontend/app/components/Profile/DangerZone.vue` - New component
- `frontend/app/pages/profile.vue` - Add danger zone section

---

### 2. Data Export/Portability ⚠️ HIGH PRIORITY

**Legal Liability:** HIGH
**Estimated Effort:** 2-3 days

**Where Claimed:**
- Privacy Policy: "You can request a machine-readable export of your data (JSON format). Contact me and I'll provide it within 30 days."

**Current Status:** PARTIALLY IMPLEMENTED (manual only)
- No automated export functionality
- Requires email to privacy@kennwilliamson.org
- No self-service option

**Compliance Risk:**
- **GDPR Article 20** (Right to Data Portability) - should be automated and instant
- **CCPA Section 1798.100** - requires accessible data export within 45 days
- Policy promises JSON format but no system to deliver it

**Implementation Required:**
1. Backend API endpoint: `GET /backend/protected/auth/export-data`
2. Service method: `AuthService::export_user_data()` returning JSON
3. Data aggregation from all user tables:
   - User profile (email, display_name, slug, created_at, etc.)
   - All incident timers
   - All phrase suggestions
   - All phrase exclusions
   - OAuth connections (Google account info)
   - Account metadata
4. Frontend UI: "Download My Data" button in profile settings
5. Response: Downloadable JSON file with proper filename

**Files to Modify:**
- `backend/src/routes/auth.rs` - Add export endpoint
- `backend/src/services/auth/auth_service/data_export.rs` - New file
- `frontend/app/components/Profile/DataExport.vue` - New component
- `frontend/app/pages/profile.vue` - Add data export section

**JSON Structure Example:**
```json
{
  "export_date": "2025-10-04T12:00:00Z",
  "user": {
    "email": "user@example.com",
    "display_name": "John Doe",
    "slug": "johndoe",
    "created_at": "2025-01-01T00:00:00Z",
    "verified": true
  },
  "incident_timers": [...],
  "phrase_suggestions": [...],
  "phrase_exclusions": [...]
}
```

---

### 3. Password Reset Flow ⚠️ HIGH PRIORITY

**Legal Liability:** MEDIUM-HIGH
**Estimated Effort:** 2-3 days

**Where Claimed:**
- Privacy Policy: "Communication: To send important account-related messages (password resets, security alerts)"

**Current Status:** NOT IMPLEMENTED for users
- Admin password reset exists but not user-initiated
- No "Forgot Password" page
- No password reset token system
- Users who forget passwords are locked out

**Compliance Risk:**
- Privacy Policy promises password reset emails
- Standard security practice that's missing
- Accessibility issue for legitimate users

**Implementation Required:**
1. Database migration: Create `password_reset_tokens` table
   - Similar to existing `verification_tokens` table
   - Columns: token, user_id, expires_at, used_at
2. Frontend pages:
   - `frontend/app/pages/forgot-password.vue` - Email input form
   - `frontend/app/pages/reset-password.vue` - New password form with token validation
3. Backend API endpoints:
   - `POST /backend/public/auth/forgot-password` - Send reset email
   - `POST /backend/public/auth/reset-password` - Verify token and update password
4. Service methods:
   - `AuthService::send_password_reset(email)`
   - `AuthService::reset_password(token, new_password)`
5. Email template: Password reset email with time-limited token (1 hour expiry)
6. Security: Rate limit forgot password requests to prevent abuse

**Files to Create/Modify:**
- `backend/migrations/` - New `password_reset_tokens` table
- `backend/src/services/auth/auth_service/password_reset.rs` - New file
- `backend/src/routes/auth.rs` - Add forgot/reset endpoints
- `backend/src/services/email/templates/password_reset.rs` - New template
- `frontend/app/pages/forgot-password.vue` - New page
- `frontend/app/pages/reset-password.vue` - New page
- `frontend/app/pages/login.vue` - Add "Forgot Password?" link

---

## Important Gaps (Should Fix Before Production)

### 4. Security Notification Emails

**Legal Liability:** MEDIUM
**Estimated Effort:** 1-2 days

**Where Claimed:**
- Privacy Policy: "Communication: To send important account-related messages (password resets, security alerts)"

**Current Status:** PARTIALLY IMPLEMENTED
- Email verification emails work
- Email infrastructure exists (AWS SES)
- NO notifications for:
  - Password changes
  - Profile updates
  - Account deactivation
  - Suspicious activity

**Implementation Required:**
1. Expand `EmailService` trait with new methods:
   - `send_password_changed_alert(user_email)`
   - `send_profile_updated_notification(user_email)`
   - `send_account_deactivated_notification(user_email, reason)`
2. Integrate into existing handlers:
   - Password change handler (after successful change)
   - Profile update handler (after changes)
   - Admin deactivation handler
3. Email templates for each notification type

**Files to Modify:**
- `backend/src/services/email/mod.rs` - Expand trait
- `backend/src/services/email/ses_email_service.rs` - Implement new methods
- `backend/src/services/email/templates/` - New template files
- `backend/src/services/auth/auth_service/password.rs` - Add notification
- `backend/src/routes/auth.rs` - Add notification to profile update
- `backend/src/routes/admin.rs` - Add notification to deactivation

---

### 5. Account Termination Notifications

**Legal Liability:** MEDIUM
**Estimated Effort:** 1 day

**Where Claimed:**
- Terms of Service: "I reserve the right to suspend or terminate your access to the site if you violate these terms... I'll try to give you notice when practical"

**Current Status:** PARTIALLY IMPLEMENTED
- Admin can deactivate accounts (`active = false`)
- Deactivated users cannot log in
- NO notification sent to user
- NO reason field or explanation

**Implementation Required:**
1. Database migration: Add `deactivation_reason` column to `users` table (TEXT, nullable)
2. Update admin deactivation endpoint to accept reason parameter
3. Send email notification when account is deactivated (use email service from Gap #4)
4. Frontend: Add reason input to admin deactivation modal
5. Login page: Show helpful message for deactivated users with contact info

**Files to Modify:**
- `backend/migrations/` - Add deactivation_reason column
- `backend/src/routes/admin.rs` - Update deactivate endpoint
- `backend/src/services/email/templates/account_deactivated.rs` - New template
- `frontend/app/pages/login.vue` - Handle deactivated user message
- Admin frontend (when built) - Add reason input field

---

### 6. Account Deletion Confirmation Email

**Legal Liability:** MEDIUM
**Estimated Effort:** Low (part of Gap #1)

**Where Claimed:**
- Implied by Privacy Policy's promise of 30-day deletion window

**Current Status:** NOT IMPLEMENTED (part of deletion feature)

**Implementation Required:**
1. Send confirmation email when user initiates deletion
2. Include:
   - Deletion scheduled date (30 days from request)
   - What will be deleted
   - Cancellation link (cancel deletion during grace period)
   - Contact information if questions
3. Add cancellation endpoint: `POST /backend/protected/auth/cancel-deletion`

**Files to Modify:**
- `backend/src/services/email/templates/account_deletion_scheduled.rs` - New template
- `backend/src/services/auth/auth_service/account.rs` - Send email on deletion
- `backend/src/routes/auth.rs` - Add cancel-deletion endpoint

---

## Nice to Have Gaps (Can Defer)

### 7. Policy Change Notification System

**Legal Liability:** LOW-MEDIUM
**Estimated Effort:** 2 days

**Where Claimed:**
- Privacy Policy: "If I make significant changes, I'll update the 'Last Updated' date at the top and notify you via email if you have an account."
- Terms of Service: Same promise

**Current Status:** NOT IMPLEMENTED
- No version tracking
- No notification system
- Manual process required

**Deferral Justification:**
- Personal site with infrequent policy changes
- Can be handled manually initially
- Low frequency of need

**Future Implementation:**
- Database table for policy versions
- Admin interface to trigger notifications
- Mass email to all active users

---

### 8. Content Removal Notifications

**Legal Liability:** LOW
**Estimated Effort:** 1-2 days

**Where Claimed:**
- Terms of Service: "I reserve the right to remove content that violates these terms or is otherwise problematic."

**Current Status:** PARTIALLY IMPLEMENTED
- Admin can deactivate phrases
- NO notification to content creator
- NO explanation provided

**Deferral Justification:**
- Content moderation is currently minimal
- Low volume of problematic content expected
- Can be handled case-by-case initially

**Future Implementation:**
- Email notification when phrase deactivated
- Reason field for deactivation
- Appeal process

---

### 9. Login Attempt Notifications

**Legal Liability:** LOW
**Estimated Effort:** Medium

**Current Status:** NOT IMPLEMENTED
- No device tracking
- No session logging
- No suspicious activity detection

**Deferral Justification:**
- Not explicitly promised in policies
- Enhanced security feature beyond current needs
- Requires significant infrastructure (device fingerprinting, geo-tracking)

---

## Implementation Roadmap

### Phase 1: Critical Compliance (2 weeks) - REQUIRED BEFORE PRODUCTION

**Week 1:**
- [ ] Account deletion functionality (3 days)
- [ ] Data export functionality (2 days)

**Week 2:**
- [ ] Password reset flow (3 days)
- [ ] Security notification emails (2 days)

**Deliverables:**
- Users can delete accounts through profile settings
- Users can export data as JSON
- Users can reset forgotten passwords
- Users receive email alerts for security-relevant changes

### Phase 2: Important Enhancements (1 week) - STRONGLY RECOMMENDED

**Week 3:**
- [ ] Account termination notifications (1 day)
- [ ] Account deletion confirmation emails (1 day)
- [ ] Content removal notifications (1 day)
- [ ] Testing and bug fixes (2 days)

**Deliverables:**
- Users notified when accounts are terminated with reason
- Deletion confirmation email with cancellation option
- Basic content moderation notifications

### Phase 3: Future Enhancements (Deferred)

**To be scheduled:**
- Policy change notification system
- Login attempt monitoring
- Advanced security features

---

## Risk Assessment Summary

### Legal Compliance Risk: HIGH

**GDPR Violations:**
- Missing Right to Erasure (Article 17) - €20M or 4% revenue fine potential
- Missing Right to Data Portability (Article 20) - €20M or 4% revenue fine potential

**CCPA Violations:**
- Missing deletion right - $7,500 per intentional violation
- Missing data portability - $7,500 per intentional violation

**Contract Law:**
- Privacy Policy and Terms of Service are legally binding contracts
- Current implementation fails to deliver promised features
- Could constitute breach of contract or false advertising

### Recommended Action

**DO NOT DEPLOY TO PRODUCTION** until Phase 1 (Critical Compliance) is complete.

The current legal documents make explicit promises that create enforceable obligations. Deploying with these gaps exposes the site to:
1. Regulatory enforcement actions (GDPR/CCPA)
2. User complaints and potential lawsuits
3. Loss of trust and reputation damage

### Alternative: Revise Legal Documents

If deployment timeline is urgent, consider:
1. Remove specific promises about unimplemented features
2. Change "You can delete your account through profile settings" to "Contact privacy@kennwilliamson.org to request account deletion"
3. Change automated data export promise to manual request process
4. Update security alert promises to reflect current capabilities

**However:** This approach is NOT RECOMMENDED because:
- Still doesn't achieve GDPR/CCPA compliance
- Degrades user experience
- Makes site less competitive
- Doesn't align with industry best practices

**RECOMMENDATION:** Implement Phase 1 features before production deployment.

---

## Technical Implementation Notes

### Database Schema Changes Required

```sql
-- Migration 1: Soft delete support
ALTER TABLE users ADD COLUMN deleted_at TIMESTAMP NULL;
CREATE INDEX idx_users_deleted_at ON users(deleted_at);

-- Migration 2: Account deactivation reasons
ALTER TABLE users ADD COLUMN deactivation_reason TEXT NULL;

-- Migration 3: Password reset tokens
CREATE TABLE password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token VARCHAR(255) NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL,
    used_at TIMESTAMP NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_password_reset_tokens_token ON password_reset_tokens(token);
CREATE INDEX idx_password_reset_tokens_user_id ON password_reset_tokens(user_id);
```

### API Endpoints to Add

**Authentication:**
- `DELETE /backend/protected/auth/delete-account` - Initiate account deletion
- `POST /backend/protected/auth/cancel-deletion` - Cancel scheduled deletion
- `GET /backend/protected/auth/export-data` - Export user data as JSON
- `POST /backend/public/auth/forgot-password` - Request password reset
- `POST /backend/public/auth/reset-password` - Reset password with token

**Email Templates:**
- Account deletion scheduled
- Account deletion cancelled
- Password reset request
- Password changed notification
- Profile updated notification
- Account deactivated notification

### Frontend Components to Create

- `Profile/DangerZone.vue` - Account deletion UI
- `Profile/DataExport.vue` - Data export button
- `pages/forgot-password.vue` - Forgot password form
- `pages/reset-password.vue` - Reset password form
- Admin deactivation reason input (future)

---

## Testing Checklist (Before Production)

### Critical Features
- [ ] Account deletion schedules soft delete correctly
- [ ] Deleted accounts cannot log in
- [ ] Deletion grace period works (30 days)
- [ ] Permanent deletion removes all user data
- [ ] Data export includes all user information
- [ ] Data export JSON is valid and complete
- [ ] Forgot password sends email
- [ ] Password reset token expires correctly
- [ ] Password reset works with valid token
- [ ] Password reset fails with invalid/expired token

### Email Notifications
- [ ] Account deletion confirmation sent
- [ ] Password reset email sent
- [ ] Password changed notification sent
- [ ] Profile updated notification sent
- [ ] Account deactivation notification sent
- [ ] All emails render correctly
- [ ] All email links work

### Security
- [ ] Rate limiting on forgot password endpoint
- [ ] Password reset tokens are single-use
- [ ] Deleted accounts data is properly isolated
- [ ] Data export only returns user's own data
- [ ] Account deletion requires authentication

---

## Questions for Resolution

1. **Data Retention Policy:** What happens to user-created public content (incident timers, phrases) when account is deleted?
   - Option A: Delete all content (clean break)
   - Option B: Anonymize content (preserve public timers/phrases but remove attribution)
   - **Recommendation:** Add clarification to Privacy Policy about this

2. **Deletion Grace Period:** Should users be able to cancel deletion?
   - Current policy promises 30-day window but doesn't mention cancellation
   - **Recommendation:** Allow cancellation and add to policy

3. **Password Reset Token Expiry:** How long should reset tokens be valid?
   - **Recommendation:** 1 hour (industry standard)

4. **Data Export Format:** JSON structure - should it be nested or flat?
   - **Recommendation:** Nested for better organization

5. **Email Sender Address:** What email should notifications come from?
   - Options: noreply@kennwilliamson.org, security@kennwilliamson.org
   - **Recommendation:** Use noreply@ for automated emails, security@ for security-related

---

## Conclusion

The Privacy Policy and Terms of Service are well-written and comprehensive, but they promise features that don't exist yet. This creates legal liability that must be addressed before production deployment.

**Minimum viable compliance** requires implementing all Phase 1 features (account deletion, data export, password reset). This is estimated at 2 weeks of development work.

**Recommended approach:** Complete Phase 1 and Phase 2 before deploying to production with Google OAuth in production mode. This ensures full compliance with GDPR, CCPA, and the site's own policies.

The alternative—deploying without these features—creates unacceptable legal risk and violates the trust established by the legal documents.
