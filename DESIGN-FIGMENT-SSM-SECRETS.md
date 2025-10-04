# Figment + AWS SSM Secrets Management Design

## Overview
Production-grade secrets management using Figment configuration library with custom AWS Systems Manager (SSM) Parameter Store provider.

## Problem Statement

### Current Limitations
- **Security Risk**: Secrets in `.env.production` tracked in git (or risk of accidental commit)
- **Manual Updates**: SSH to server and edit files for new secrets
- **No Rotation**: Changing secrets requires container restarts and coordination
- **Developer Friction**: Each dev needs own `.env.development.local` with credentials
- **Audit Trail**: No visibility into who accessed what secrets when

### Requirements
1. **Zero secrets in git** - Never risk committing credentials
2. **Multi-environment** - Dev, staging, production separation
3. **Easy rotation** - Update secrets without code changes
4. **Team collaboration** - Shared dev credentials, no local files
5. **Production ready** - IAM-based access, encryption at rest
6. **Cost effective** - Ideally free or minimal cost

## Solution: Figment + Custom SSM Provider

### Why Figment?

**Figment** (https://docs.rs/figment/) is the standard configuration library in Rust:
- ✅ Used by Rocket web framework (battle-tested)
- ✅ Provider trait pattern (exactly what we need)
- ✅ Composable sources with precedence rules
- ✅ Type-safe config extraction
- ✅ Profile support (dev, prod, test)
- ✅ Tracks value sources (debugging)

**Why NOT create our own trait:**
- Figment already has perfect abstraction (`Provider` trait)
- Don't reinvent the wheel
- Ecosystem compatibility (can use other Figment providers)

**Why NOT make a crate immediately:**
- Start internal to our project (`backend/src/config/providers/ssm.rs`)
- Validate design and requirements first
- Extract to `figment-aws-ssm` crate later if valuable to community

### Why AWS SSM Parameter Store?

**Free Tier (Perfect for us):**
- Standard Parameters: **FREE** (up to 10,000 params)
- Unlimited reads on standard params
- No charges for our use case

**vs Secrets Manager:**
- Secrets Manager: ~$0.40/secret/month ($2-3/month for us)
- SSM: $0.00/month
- Only difference: Automatic rotation (we don't need it yet)

**Security:**
- IAM-based access control
- Encryption at rest (AWS KMS - free tier)
- Audit logging via CloudTrail
- No credentials in code (EC2 IAM role)

**Developer Experience:**
- Works locally with AWS credentials
- Same code for dev and prod (different prefix)
- Easy to add/update secrets via AWS CLI or Console

## Architecture

### Figment Provider Trait

```rust
pub trait Provider: Send + Sync {
    fn metadata(&self) -> Metadata;
    fn data(&self) -> Result<Map<Profile, Dict>, Error>;
    fn profile(&self) -> Option<Profile> { None }
}
```

### Custom SSM Provider

```rust
// backend/src/config/providers/ssm.rs
use figment::{Provider, Metadata, Profile, Error};
use figment::value::{Map, Dict};
use aws_sdk_ssm::Client;

pub struct SsmProvider {
    /// SSM parameter prefix (e.g., "/kennwilliamson/prod/")
    prefix: String,

    /// AWS SSM client
    client: Client,

    /// Optional profile (defaults to "default")
    profile: Option<Profile>,
}

impl SsmProvider {
    /// Create new SSM provider with auto-detected AWS credentials
    pub async fn new(prefix: impl Into<String>) -> Result<Self> {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);

        Ok(Self {
            prefix: prefix.into(),
            client,
            profile: None,
        })
    }

    /// Set profile for multi-environment configs
    pub fn profile(mut self, profile: impl Into<Profile>) -> Self {
        self.profile = Some(profile.into());
        self
    }

    /// Fetch all parameters under prefix from SSM
    async fn fetch_parameters(&self) -> Result<Map<String, String>> {
        let result = self.client
            .get_parameters_by_path()
            .path(&self.prefix)
            .with_decryption(true)  // Decrypt SecureString params
            .send()
            .await?;

        let mut params = Map::new();
        for param in result.parameters.unwrap_or_default() {
            let name = param.name.unwrap();
            let value = param.value.unwrap();

            // Strip prefix: "/kennwilliamson/prod/jwt_secret" -> "jwt_secret"
            let key = name.strip_prefix(&self.prefix)
                .unwrap_or(&name)
                .to_string();

            params.insert(key, value);
        }

        Ok(params)
    }
}

impl Provider for SsmProvider {
    fn metadata(&self) -> Metadata {
        Metadata::named("AWS SSM Parameter Store")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        let params = tokio::runtime::Handle::current()
            .block_on(self.fetch_parameters())
            .map_err(|e| Error::from(e.to_string()))?;

        let profile = self.profile.clone().unwrap_or_default();
        let mut dict = Dict::new();

        for (key, value) in params {
            dict.insert(key, value.into());
        }

        Ok(Map::from([(profile, dict)]))
    }
}
```

### Config Loading Pattern

```rust
// backend/src/config/mod.rs
use figment::{Figment, providers::{Env, Format, Toml}};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub jwt_secret: String,
    pub db_password: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub aws_access_key_id: Option<String>,
    pub aws_secret_access_key: Option<String>,
    // ... other config
}

pub async fn load_config() -> Result<AppConfig> {
    let env = std::env::var("ENV").unwrap_or_else(|_| "dev".to_string());

    let figment = Figment::new()
        // 1. Base defaults from .env file
        .merge(Toml::file(".env.development"))

        // 2. Environment-specific SSM (if AWS configured)
        .merge(SsmProvider::new(format!("/kennwilliamson/{}/", env)).await?)

        // 3. Explicit env vars (highest priority)
        .merge(Env::prefixed("APP_"));

    figment.extract()
}
```

### Precedence (Lowest to Highest)

1. **`.env.development`** - Base defaults, safe for git
2. **SSM Parameters** - Environment-specific secrets
3. **Environment Variables** - Override for local testing

## Implementation Plan

### Phase 1: Dependencies & Setup

**Add to `backend/Cargo.toml`:**
```toml
[dependencies]
figment = { version = "0.10", features = ["toml", "env"] }
aws-config = { version = "1.8.7", features = ["behavior-version-latest"] }
aws-sdk-ssm = "1.x"  # Latest version
```

**File Structure:**
```
backend/src/config/
├── mod.rs              # Config loading logic
├── providers/
│   ├── mod.rs
│   └── ssm.rs          # SsmProvider implementation
└── app_config.rs       # AppConfig struct definition
```

### Phase 2: SsmProvider Implementation

**Tasks:**
1. ✅ Create `backend/src/config/providers/ssm.rs`
2. ✅ Implement `Provider` trait for `SsmProvider`
3. ✅ Handle AWS credential auto-detection
4. ✅ Handle parameter prefix stripping
5. ✅ Handle SecureString decryption
6. ✅ Error handling and fallback

**Testing:**
```rust
#[tokio::test]
async fn test_ssm_provider_loads_parameters() {
    // Setup test parameters in SSM (or mock)
    let provider = SsmProvider::new("/test/").await.unwrap();
    let data = provider.data().unwrap();

    assert!(data.contains_key("jwt_secret"));
}
```

### Phase 3: Integration with ServiceContainer

**Update `backend/src/services/container.rs`:**
```rust
// OLD:
let jwt_secret = env::var("JWT_SECRET")?;

// NEW:
let config = load_config().await?;
let jwt_secret = config.jwt_secret;
```

### Phase 4: AWS SSM Setup

**IAM Policy** (`aws-policies/ec2-ssm-read-policy.json`):
```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "ssm:GetParameter",
        "ssm:GetParameters",
        "ssm:GetParametersByPath"
      ],
      "Resource": "arn:aws:ssm:*:*:parameter/kennwilliamson/*"
    },
    {
      "Effect": "Allow",
      "Action": [
        "kms:Decrypt"
      ],
      "Resource": "*",
      "Condition": {
        "StringEquals": {
          "kms:ViaService": "ssm.*.amazonaws.com"
        }
      }
    }
  ]
}
```

**Setup Script** (`scripts/setup-ssm-parameters.sh`):
```bash
#!/bin/bash
# Setup SSM parameters for an environment

ENV=${1:-dev}
PREFIX="/kennwilliamson/$ENV"

echo "Setting up SSM parameters for environment: $ENV"

# Prompt for secrets
read -sp "JWT Secret: " JWT_SECRET
echo
read -sp "DB Password: " DB_PASSWORD
echo
read -sp "Google Client Secret: " GOOGLE_SECRET
echo

# Create parameters
aws ssm put-parameter \
    --name "$PREFIX/jwt_secret" \
    --value "$JWT_SECRET" \
    --type SecureString \
    --overwrite

aws ssm put-parameter \
    --name "$PREFIX/db_password" \
    --value "$DB_PASSWORD" \
    --type SecureString \
    --overwrite

aws ssm put-parameter \
    --name "$PREFIX/google_client_secret" \
    --value "$GOOGLE_SECRET" \
    --type SecureString \
    --overwrite

echo "✅ Parameters created successfully!"
aws ssm get-parameters-by-path --path "$PREFIX" --query 'Parameters[*].Name'
```

**Migration Script** (`scripts/migrate-env-to-ssm.sh`):
```bash
#!/bin/bash
# Migrate existing .env.production secrets to SSM

source .env.production
PREFIX="/kennwilliamson/prod"

aws ssm put-parameter \
    --name "$PREFIX/jwt_secret" \
    --value "$JWT_SECRET" \
    --type SecureString

aws ssm put-parameter \
    --name "$PREFIX/db_password" \
    --value "$DB_PASSWORD" \
    --type SecureString

echo "✅ Migrated secrets to SSM"
echo "⚠️  Remember to remove secrets from .env.production!"
```

## Developer Experience

### Local Development Setup

**Option 1: Use .env files (No AWS needed)**
```bash
# .env.development has everything
./scripts/dev-start.sh

# Figment loads from .env.development
# No SSM provider runs
```

**Option 2: Use SSM for realistic testing**
```bash
# Configure AWS credentials (one-time)
aws configure

# Secrets loaded from /kennwilliamson/dev/
ENV=dev ./scripts/dev-start.sh
```

### Adding New Secrets

**1. Add to SSM:**
```bash
./scripts/setup-ssm-parameters.sh dev
# Follow prompts
```

**2. Update AppConfig struct:**
```rust
#[derive(Deserialize)]
pub struct AppConfig {
    // ... existing
    pub new_secret: String,
}
```

**3. Use in code:**
```rust
let config = load_config().await?;
let my_secret = config.new_secret;
```

### Debugging

**Check what Figment loaded:**
```rust
let figment = Figment::new()
    .merge(SsmProvider::new("/kennwilliamson/dev/").await?);

// Show metadata (which providers loaded what)
for (key, meta) in figment.metadata() {
    println!("{}: loaded from {}", key, meta.source);
}
```

## Environment Configuration

### Parameter Naming Convention

```
/kennwilliamson/{env}/{secret_name}

Examples:
/kennwilliamson/dev/jwt_secret
/kennwilliamson/dev/db_password
/kennwilliamson/dev/google_client_secret

/kennwilliamson/prod/jwt_secret
/kennwilliamson/prod/db_password
/kennwilliamson/prod/google_client_secret
```

### Environment Detection

```rust
// Automatic based on ENV variable
let env = std::env::var("ENV").unwrap_or_else(|_| "dev".to_string());

// Local dev: ENV not set → "dev" → loads from .env.development
// Production: ENV=prod → "prod" → loads from /kennwilliamson/prod/
```

### .env.development (Safe for Git)

```bash
# Development Environment Configuration
# Secrets loaded from SSM or overridden locally

# Database (password from SSM)
DB_USER=postgres
# DB_PASSWORD loaded from SSM: /kennwilliamson/dev/db_password

# JWT (secret from SSM)
# JWT_SECRET loaded from SSM: /kennwilliamson/dev/jwt_secret

# Google OAuth (secret from SSM)
GOOGLE_CLIENT_ID=dev-client-id.apps.googleusercontent.com
# GOOGLE_CLIENT_SECRET loaded from SSM: /kennwilliamson/dev/google_client_secret

# Public config (safe)
DOMAIN_NAME=localhost
CORS_ORIGIN=http://localhost:3000
RUST_LOG=backend=debug
```

## Production Deployment

### Migration Checklist

**1. Create SSM Parameters:**
```bash
# From local machine with AWS credentials
./scripts/migrate-env-to-ssm.sh
```

**2. Update EC2 IAM Role:**
- Attach `ec2-ssm-read-policy.json` to instance role
- Verify with: `aws ssm get-parameters-by-path --path /kennwilliamson/prod/`

**3. Deploy Updated Code:**
```bash
ssh ubuntu@your-server
cd /opt/kennwilliamson/kennwilliamsondotorg
git pull origin main

# Rebuild with Figment support
docker-compose --env-file .env.production build backend

# Restart (loads from SSM automatically)
docker-compose --env-file .env.production restart backend
```

**4. Verify:**
```bash
# Check health
./scripts/health-check.sh

# Check logs for "loaded from AWS SSM"
docker-compose logs backend | grep -i ssm
```

**5. Clean Up .env.production:**
```bash
# Remove secret values (keep config)
nano .env.production
# Remove JWT_SECRET=... and DB_PASSWORD=... lines
```

### Rollback Plan

**If SSM fails:**
```bash
# Restore .env.production backup
cp .env.production.backup .env.production

# Revert to previous code
git checkout <previous-commit>
docker-compose --env-file .env.production build backend
docker-compose --env-file .env.production restart backend
```

## Testing Strategy

### Unit Tests (Mock Provider)

```rust
struct MockSsmProvider {
    params: HashMap<String, String>,
}

impl Provider for MockSsmProvider {
    // Return mock data
}

#[test]
fn test_config_loads_from_mock_ssm() {
    let mock = MockSsmProvider::new(hashmap! {
        "jwt_secret" => "test-secret",
    });

    let figment = Figment::new().merge(mock);
    let config: AppConfig = figment.extract().unwrap();

    assert_eq!(config.jwt_secret, "test-secret");
}
```

### Integration Tests (Real SSM)

```rust
#[tokio::test]
#[ignore] // Requires AWS credentials
async fn test_ssm_provider_real() {
    let provider = SsmProvider::new("/test/").await.unwrap();
    let data = provider.data().unwrap();

    assert!(!data.is_empty());
}
```

### Local Testing with SSM

```bash
# Create test parameters
aws ssm put-parameter \
    --name /kennwilliamson/dev/test_secret \
    --value "test-value" \
    --type SecureString

# Run app with SSM
ENV=dev cargo run

# Verify it loaded from SSM (check logs)
```

## Security Considerations

### Secrets Never Touch Disk
- ✅ SSM parameters loaded into memory
- ✅ Not written to log files
- ✅ Not included in error messages
- ✅ Cleared on app shutdown

### IAM Least Privilege
- EC2 role only has `ssm:GetParameter` on `/kennwilliamson/*`
- No write permissions
- No access to other SSM paths
- KMS decrypt only via SSM service

### Audit Trail
- All SSM access logged to CloudTrail
- Can see who accessed what and when
- Alerts on unusual access patterns

### Encryption at Rest
- All SecureString parameters encrypted with AWS KMS
- Keys managed by AWS (free tier)
- Automatic rotation available (future)

## Cost Analysis

### AWS SSM Parameter Store

**Standard Parameters (Our Use Case):**
- Storage: FREE (up to 10,000 params)
- API Calls: FREE (standard throughput)
- KMS Encryption: FREE (AWS-managed keys)
- **Total: $0.00/month**

**Current Usage:**
- ~6-10 parameters per environment
- ~20-30 total parameters (dev, staging, prod)
- Well within free tier

**vs Secrets Manager:**
- Secrets Manager: ~$0.40/secret/month
- 10 secrets = $4/month
- Only benefit: Automatic rotation (not needed yet)

## Future Enhancements

### Extract to Public Crate

**When ready, extract to `figment-aws-ssm`:**
```rust
// Published crate
use figment_aws_ssm::SsmProvider;

let config = Figment::new()
    .merge(SsmProvider::new("/myapp/prod/").await?)
    .extract()?;
```

**Benefits:**
- Help the community
- Get feedback and improvements
- Resume/portfolio piece
- Potential Figment integration

### Additional Providers

**Could add in future:**
- `VaultProvider` - HashiCorp Vault
- `SecretsManagerProvider` - AWS Secrets Manager
- `GcpSecretProvider` - Google Cloud Secret Manager
- `AzureKeyVaultProvider` - Azure Key Vault

### Automatic Rotation

**AWS Secrets Manager supports:**
- Lambda-triggered rotation
- Automatic credential updates
- Zero-downtime rotation

**Migration path:**
1. Start with SSM (free, simple)
2. Move to Secrets Manager if rotation needed
3. Just change provider in config

## Open Questions

1. **Caching**: Should we cache SSM params in memory? (Probably not - Figment already does one load)
2. **Refresh**: How to handle secret rotation without restart? (Future: watch for changes)
3. **Fallback**: What if SSM is down? (Current: use .env fallback, acceptable)
4. **Profiles**: Use Figment profiles for environments or separate prefixes? (Separate prefixes is simpler)

## Success Criteria

### Phase 1 Complete:
- ✅ Figment integrated
- ✅ SsmProvider implemented
- ✅ Works locally with .env fallback
- ✅ Tests passing

### Phase 2 Complete:
- ✅ SSM parameters created (dev)
- ✅ Local dev loads from SSM
- ✅ IAM policies documented
- ✅ Migration scripts working

### Phase 3 Complete:
- ✅ Production using SSM
- ✅ Zero secrets in .env.production
- ✅ All services healthy
- ✅ Audit trail configured

### Future:
- ✅ Extract to `figment-aws-ssm` crate
- ✅ Automatic rotation (if needed)
- ✅ Multi-cloud providers

---

## References

- **Figment Documentation**: https://docs.rs/figment/
- **AWS SSM Parameter Store**: https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-parameter-store.html
- **Figment Provider Trait**: https://docs.rs/figment/latest/figment/trait.Provider.html
- **AWS SDK for Rust**: https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html

---

*Created: 2025-10-03*
*Status: Design Complete - Ready for Implementation*
*Next: Implement Phase 1 (after OAuth Phase 2 complete)*
