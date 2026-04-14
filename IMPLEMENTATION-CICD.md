# CI/CD Implementation

This document explains the CI/CD (Continuous Integration / Continuous Deployment) architecture for KennWilliamson.org, following the Decision/Why/Alternatives/Trade-offs pattern.

## Overview

**Pattern**: Trunk-based development with tag-triggered deployments
- **CI**: Automated testing and validation on every push/PR to `master`
- **CD**: Automated deployment on semantic version tags (e.g., `v1.0.0`)

## Architecture Decisions

### Trunk-Based Development with Tag Deployment

**Decision**: CI runs on all commits to `master`, but deployment only triggers on version tags.

**Why**:
- **Main always deployable**: CI validates every commit, ensuring master branch is always in a releasable state
- **Deliberate releases**: Deployments happen when you're ready, not automatically on every merge
- **Easy rollback**: Simple to roll back by tagging a previous commit
- **Professional workflow**: Demonstrates understanding of production Git workflows for portfolio
- **Cost-effective**: No staging environment needed - CI provides quality gates

**Alternatives rejected**:
- **Deploy on every merge**: Too aggressive for single-developer project, could deploy incomplete features
- **Manual deployment only**: Misses opportunity for automation and consistency
- **Feature branch deployment**: Requires staging environment, overkill for project scale

**Trade-offs**:
- ✅ Main branch quality enforced through CI
- ✅ Flexible deployment timing
- ✅ Simple rollback via tags
- ❌ Need to remember to tag for deployment
- ❌ No preview deployments for feature branches

---

### GitHub Container Registry (ghcr.io)

**Decision**: Use GitHub Container Registry instead of building images on EC2.

**Why**:
- **Free**: Unlimited public images, generous private image allowance
- **Integrated**: Native GitHub Actions integration, no separate auth setup
- **Fast**: Same network as GitHub runners, quick push/pull
- **Versioned**: Each tag creates immutable, versioned artifact
- **Faster deployments**: Pre-built images deploy faster than rebuilding on EC2

**Alternatives rejected**:
- **AWS ECR**: More expensive, requires AWS credential management in GitHub
- **Docker Hub**: Separate service, rate limiting on free tier, slower pulls
- **Build on EC2**: Slower deployments, uses production server resources, harder rollback

**Trade-offs**:
- ✅ Free and fast
- ✅ Simple authentication
- ✅ Version history maintained
- ✅ Faster deployments
- ❌ Vendor lock-in to GitHub (mitigated: Docker images are portable)
- ❌ Public images visible (acceptable: open-source project)

---

### CI Pipeline Components

**Decision**: Five parallel CI jobs - backend tests, backend clippy, backend audit, frontend tests, Docker builds.

**Why**:
- **Parallel execution**: Faster CI completion (jobs run concurrently)
- **Clear failure isolation**: Know immediately which component failed
- **Comprehensive validation**: Tests, linting, security scans, build validation
- **Portfolio value**: Demonstrates understanding of CI best practices

**Components**:

#### Backend Tests (backend-tests job)
1. **Rust tests with coverage** using cargo-llvm-cov + cargo-nextest (CI profile, LCOV output)
2. **PostgreSQL + Redis services** (integration test dependencies)
3. **SQLx migrations** run before tests

**Why**: Full test suite with coverage reporting, catches regressions and measures test quality

#### Backend Clippy (backend-clippy job)
1. **Clippy linting** with `--all-targets --all-features` (fail on warnings)

**Why**: Separate job for fast failure isolation. Enforces Rust best practices, catches common mistakes at compile time. The `--all-targets` flag ensures tests, benches, and examples are also linted.

#### Backend Audit (backend-audit job)
1. **cargo audit** with `--deny warnings` (security vulnerability scanning)

**Why**: Separate job so security scanning runs independently. Protects users from known security vulnerabilities in dependencies.

#### Frontend Tests (frontend-tests job)
1. **TypeScript type checking** (vue-tsc, no compilation, type safety only)
2. **Vitest tests with coverage** (unit and integration tests)
3. **npm audit** (security vulnerability scanning, high severity)

**Why these checks**:
- Type checking: Catches type errors before runtime, demonstrates TypeScript usage
- Vitest: Fast test runner for Nuxt/Vue, modern tooling
- npm audit: Frontend security scanning (set to high severity to avoid noise)

#### Docker Build CI (docker-build job)
1. **Build backend Docker image** (validates production build process)
2. **Build frontend Docker image** (validates production build process)
3. **GitHub Actions cache** (speeds up subsequent builds)
4. **Only runs on PRs** (skipped on master pushes to save resources)

**Why Docker build validation**:
- Catch Docker build failures early (before attempting deployment)
- Validates SQLx cache is up-to-date (backend)
- Validates TypeScript compilation (frontend)
- Uses GitHub Actions cache for faster subsequent builds

#### Auxiliary Jobs
- **codecov-upload**: Uploads backend + frontend coverage to Codecov (PR only)
- **ci-success**: Gate job that fails if any required job fails (used for branch protection)

**Alternatives rejected**:
- **ESLint in CI**: Not configured in project, would require setup first (can add later)
- **E2E tests**: Slower, more brittle, existing test suite provides good coverage
- **Docker image scanning**: Adds complexity, less critical for low-traffic personal site

**Trade-offs**:
- ✅ Fast feedback (parallel jobs)
- ✅ Comprehensive coverage
- ✅ Security scanning included
- ❌ Requires maintaining test suite
- ❌ CI runtime costs (mitigated: GitHub Actions free tier for public repos)

---

### CD Pipeline Components

**Decision**: Build → Push → Deploy → Verify workflow triggered by semantic version tags.

**Why**:
- **Immutable artifacts**: Each release creates versioned Docker images
- **Atomic deployment**: All services updated together to matching version
- **Automated verification**: Health checks ensure deployment succeeded
- **Automatic rollback**: Failures trigger rollback workflow

**Components**:

#### 1. Build & Push Job
- **Triggers**: Tag push matching `v*.*.*` (e.g., `v1.0.0`, `v2.1.3`)
- **Builds**: Backend, frontend, migrations Docker images
- **Tags**: Both versioned tag (`v1.0.0`) and `latest`
- **Pushes**: All images to `ghcr.io/<user>/kennwilliamsondotorg-*`
- **Cache**: GitHub Actions cache for faster builds

**Why versioned + latest tags**:
- Versioned: Pin specific release, essential for rollback
- Latest: Convenience for quick deployments/testing

#### 2. Deploy Job
- **SSH to EC2**: Uses GitHub Secrets for credentials
- **Copy files**: `docker-compose.production.yml` and `deploy-from-registry.sh`
- **Run deployment**: Executes `deploy-from-registry.sh` with version
- **Health checks**: Verifies backend/frontend health endpoints
- **Rollback on failure**: Automatic rollback if health checks fail

**Why SSH deployment**:
- Simple and reliable
- Works with existing EC2 infrastructure
- No additional AWS services needed
- Easy to debug (can manually SSH and inspect)

#### 3. Notify Job
- **Success**: Log success message with version and URL
- **Failure**: Log failure message, exit with error code

**Future enhancement**: Add Slack/Discord/email notifications

**Alternatives rejected**:
- **AWS CodeDeploy**: Over-engineered for single EC2 instance, added complexity
- **Blue-green deployment**: Requires 2x resources, overkill for low-traffic site
- **Kubernetes**: Massive overkill for this scale, increases costs and complexity
- **Docker Swarm**: Unnecessary orchestration for single-server deployment

**Trade-offs**:
- ✅ Simple and reliable
- ✅ Automated rollback on failure
- ✅ Version history via tags
- ✅ Fast deployments (pre-built images)
- ❌ Brief downtime during container restart (~30s)
- ❌ Single EC2 instance (no high availability)
- ❌ No blue-green deployment

---

### Deployment Strategy

**Decision**: Container registry deployment with pull-based updates.

**Process**:
1. GitHub Actions builds versioned images
2. Images pushed to ghcr.io with version tag
3. SSH to EC2 production server
4. Pull versioned images from ghcr.io
5. Stop existing containers gracefully (30s timeout)
6. Start new containers with pulled images
7. Run database migrations
8. Health check verification
9. Rollback on failure

**Why pull-based deployment**:
- **Faster**: Pre-built images faster than rebuilding on server
- **Consistent**: Same images validated in CI
- **Version pinning**: Explicit version control via tags
- **Easy rollback**: Pull previous version, restart containers

**Deployment Scripts**:

#### `scripts/deploy-from-registry.sh`
- Pull images from ghcr.io with specified version
- Gracefully stop existing containers
- Start new containers
- Run migrations
- Cleanup old images (save disk space)
- Comprehensive logging

#### `scripts/rollback.sh`
- Pull specific previous version
- Restart containers with rollback version
- Run migrations (handle schema differences)
- Confirmation prompts (prevent accidental rollback)
- Audit logging

**Alternatives rejected**:
- **Build on server**: Slower, uses production resources, harder to rollback
- **Git-based deployment**: Original pattern, slower than pulling pre-built images
- **Docker Hub**: Slower, rate-limited, less integrated than ghcr.io

**Trade-offs**:
- ✅ Fast deployments (30-60 seconds)
- ✅ Easy rollback (few commands)
- ✅ Version control via tags
- ❌ Requires container registry
- ❌ Brief downtime during restart

---

## Test Coverage Reporting

### Decision: Automated Coverage Reports on Pull Requests

**Decision**: Use cargo-llvm-cov (Rust) + Vitest coverage (Frontend) + Codecov for automated PR coverage reports.

**Why**:
- **Visibility**: Developers see exactly which lines are/aren't covered in PRs
- **Accountability**: Diff coverage shows coverage of changed code specifically
- **Trends**: Track coverage over time, identify coverage regressions
- **Portfolio value**: Demonstrates testing rigor and quality practices
- **Free**: Codecov free tier unlimited for public repos

**Components**:

#### Backend Coverage: cargo-llvm-cov
- Uses LLVM instrumentation for accurate coverage
- Runs on stable Rust with `llvm-tools-preview` component
- Integrates with cargo-nextest for fast test execution
- Generates LCOV format for Codecov
- Integrated into `backend-tests` job

**Command**: `cargo llvm-cov nextest --lcov --output-path coverage/lcov.info --all-features`

#### Frontend Coverage: Vitest + @vitest/coverage-v8
- Already configured in project
- Istanbul coverage provider
- Generates LCOV format
- Integrated into `frontend-tests` job

**Command**: `npm run test:coverage`

#### Codecov Integration
- Codecov Action v5 (latest)
- Uploads both backend + frontend coverage in single report
- Automatic PR comments (no token needed for public repos)
- Flags for backend/frontend separation
- Informational only (doesn't block PRs)

**Configuration**: `codecov.yml` at project root

**Alternatives rejected**:
- **cargo-tarpaulin**: Simpler setup but less accurate, no branch coverage support
- **grcov**: Requires nightly Rust, more complex
- **Coveralls**: Less feature-rich than Codecov
- **Manual coverage**: No automation, no trend tracking

**Trade-offs**:
- ✅ Automated PR feedback on coverage
- ✅ Coverage trends tracked over time
- ✅ Free for public repos
- ✅ Beautiful visualizations
- ✅ Accurate LLVM-based instrumentation with branch coverage support
- ❌ Requires llvm-tools-preview component
- ❌ External service dependency (Codecov)

### Coverage Thresholds

**Project Coverage** (informational):
- Target: Auto (based on previous commits)
- Threshold: 1% decrease allowed
- Status: Informational (doesn't block PRs)

**Patch Coverage** (diff coverage):
- Target: 80% of changed lines covered
- Threshold: 5% flexibility
- Status: Informational (doesn't block PRs)

**Philosophy**: Coverage reports inform, but don't block. Encourage good testing practices without creating merge friction.

### Interpreting Codecov Reports

**PR Comments include**:
1. **Overall coverage**: Current coverage % vs. main branch
2. **Diff coverage**: % of lines changed in PR that are covered
3. **File breakdown**: Per-file coverage changes
4. **Trend**: Coverage increase/decrease

**Example PR Comment**:
```
Codecov Report
Coverage: 78.45% (+2.15% 📈)
Diff Coverage: 85.32%

Files Changed:
├─ backend/src/services/auth.rs: 92.3% (+5.2% 📈)
├─ frontend/app/composables/useAuth.ts: 81.5% (-1.2% 📉)

[View detailed report →]
```

### Local Coverage Generation

**Backend**:
```bash
cd backend
cargo install cargo-llvm-cov  # One-time install
cargo llvm-cov --html
# Open target/llvm-cov/html/index.html in browser
```

**Frontend**:
```bash
cd frontend
npm run test:coverage
# Open coverage/index.html in browser
```

**Coverage artifacts** (gitignored):
- `backend/coverage/` - LCOV reports
- `backend/target/llvm-cov/` - HTML reports
- `frontend/coverage/` - Istanbul reports

---

## CI/CD Workflows

### CI Workflow (`.github/workflows/ci.yml`)

**Triggers**:
- Push to `master` branch
- Pull requests to `master` branch

**Jobs**:
1. `backend-tests`: Rust tests with coverage (cargo-llvm-cov + nextest)
2. `backend-clippy`: Clippy lints with `--all-targets`
3. `backend-audit`: Security vulnerability scanning (cargo audit)
4. `frontend-tests`: Vitest tests with coverage, TypeScript check, npm audit
5. `docker-build`: Build validation for backend/frontend Docker images (PRs only)
6. `codecov-upload`: Upload coverage reports to Codecov (PR only)
7. `ci-success`: Gate job that fails if any other job fails

**Environment**:
- PostgreSQL service container (ghcr.io/fboulnois/pg_uuidv7:1.6.0)
- Redis 7 service container
- Rust 1.91.1 toolchain with llvm-tools-preview
- Node.js 24
- cargo-llvm-cov, cargo-nextest, cargo-audit

**Artifacts**:
- Backend coverage (LCOV format) - 30 days retention
- Frontend coverage (LCOV format) - 30 days retention
- Uploaded to Codecov on pull requests

**Coverage Features**:
- Automated PR comments with coverage reports
- Diff coverage (changed lines only)
- Coverage trend tracking
- Backend + frontend combined reports

---

### CD Workflow (`.github/workflows/deploy.yml`)

**Triggers**:
- Tag push matching `v*.*.*`

**Jobs**:
1. `build-and-push`: Build and push Docker images to ghcr.io
2. `deploy`: SSH to EC2, deploy from registry, verify health
3. `notify`: Report deployment success/failure

**Secrets Required** (GitHub Repository Settings):
- `EC2_SSH_KEY`: Private SSH key for EC2 access
- `EC2_HOST`: Production server hostname/IP
- `EC2_USER`: SSH username (e.g., `ec2-user`, `ubuntu`)

**Environment**:
- Production environment configured in GitHub
- URL: https://kennwilliamson.org

**Deployment Process**:
1. Extract version from tag (e.g., `v1.0.0`)
2. Build all images with version tag + `latest` tag
3. Push images to ghcr.io
4. SSH to EC2 and copy deployment files
5. Run `deploy-from-registry.sh` with version
6. Wait for services to stabilize (30s)
7. Verify backend health endpoint (`/health`)
8. Verify frontend health endpoint (`/api/health`)
9. Rollback on failure

---

## Release Process

### Automated Releases with Release-Please

**Decision**: Use Release-Please for automated semantic versioning based on conventional commits.

**Why**:
- **Automatic version calculation**: No manual version decisions
- **CHANGELOG generation**: Auto-generated from commit messages
- **Review before release**: PR-based workflow gives control
- **Professional process**: Industry-standard release automation

#### How It Works

1. **Develop with Conventional Commits**:
   ```bash
   git commit -m "feat: add user dashboard"
   git commit -m "fix: resolve login bug"
   git push origin master
   ```

2. **Release-Please Bot Creates PR**:
   - Triggered automatically on push to master
   - Creates/updates PR titled: "chore: release vX.Y.Z"
   - Calculates version from conventional commits:
     - `feat:` → MINOR bump (v1.1.0)
     - `fix:` → PATCH bump (v1.0.1)
     - `feat!:` or `BREAKING CHANGE:` → MAJOR bump (v2.0.0)
   - Generates CHANGELOG from commits
   - Updates PR with each new commit

3. **Review Release PR**:
   - Check version number is correct
   - Review CHANGELOG and included commits
   - Verify CI passed on all commits
   - **Merge when ready to deploy**

4. **Merge Triggers Deployment**:
   - Merge creates git tag (e.g., v1.0.0)
   - Tag triggers CD Pipeline automatically
   - Deployment proceeds as normal

#### Conventional Commit Format

**Format**: `<type>: <description>`

**Version Bumps**:
- `feat:` → MINOR (v1.1.0)
- `fix:` → PATCH (v1.0.1)
- `feat!:` → MAJOR (v2.0.0)
- `BREAKING CHANGE:` in body → MAJOR
- `docs:`, `chore:`, `style:`, `refactor:` → No bump

**Examples**:
```bash
feat: add Google OAuth login          # v1.1.0
fix: resolve timer display bug        # v1.0.1
feat!: change API response format     # v2.0.0
docs: update README                   # no version change
```

### Manual Release (Fallback)

**If Release-Please is unavailable**:

1. **Ensure master is clean**:
   ```bash
   git checkout master
   git pull origin master
   ```

2. **Verify CI passed**:
   - Check GitHub Actions for latest commit

3. **Create semantic version tag manually**:
   ```bash
   git tag v1.0.0  # Major.Minor.Patch
   git push origin v1.0.0
   ```

4. **Monitor deployment**:
   - GitHub Actions → CD Pipeline workflow
   - Watch build-and-push job (5-10 minutes)
   - Watch deploy job (2-5 minutes)
   - Verify at https://kennwilliamson.org

### Semantic Versioning Guidelines

- **MAJOR** (v2.0.0): Breaking changes (API changes, database schema incompatibilities)
- **MINOR** (v1.1.0): New features, backwards-compatible
- **PATCH** (v1.0.1): Bug fixes, small improvements

### Rollback Process

#### Option 1: Rollback via Script (Recommended)

On production server:
```bash
ssh <ec2-host>
cd /home/<user>/app
export GITHUB_USER=<your-github-username>
./scripts/rollback.sh v1.0.0  # Specify version to rollback to
```

#### Option 2: Rollback via Git Tag

Create new tag pointing to previous commit:
```bash
git checkout <previous-commit-hash>
git tag v1.0.1  # New tag for rollback version
git push origin v1.0.1
# CD pipeline deploys previous code as new version
```

**Why Option 1 is preferred**:
- Faster (no CI/CD pipeline delay)
- Direct rollback on server
- Explicit rollback intent (audit log)

**Why Option 2 is useful**:
- No server access needed
- Goes through full CI/CD validation
- Creates new version in registry

---

## Setup Instructions

### 1. GitHub Repository Configuration

#### Enable GitHub Container Registry
- Repository → Settings → Actions → General
- Workflow permissions → Read and write permissions
- Allow GitHub Actions to create and approve pull requests

#### Add Repository Secrets
Repository → Settings → Secrets and variables → Actions:

```
EC2_SSH_KEY = <private SSH key content>
EC2_HOST = <production server IP or hostname>
EC2_USER = <SSH username, e.g., ec2-user>
```

**Generating EC2 SSH Key**:
```bash
# On your local machine
ssh-keygen -t ed25519 -C "github-actions-deploy" -f ~/.ssh/github_deploy
# Copy public key to EC2
ssh-copy-id -i ~/.ssh/github_deploy.pub <ec2-user>@<ec2-host>
# Add private key to GitHub Secrets (entire content of ~/.ssh/github_deploy)
cat ~/.ssh/github_deploy  # Copy this to EC2_SSH_KEY secret
```

#### Configure Branch Protection (Optional but Recommended)
Repository → Settings → Branches → Add rule for `master`:
- Require status checks to pass before merging
- Select required checks: `backend-tests`, `frontend-tests`, `docker-build`
- Require branches to be up to date before merging

### 2. EC2 Server Setup

#### Install Docker & Docker Compose
```bash
# Install Docker
sudo yum update -y
sudo yum install docker -y
sudo service docker start
sudo usermod -a -G docker ec2-user

# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

#### Create Application Directory
```bash
mkdir -p /home/ec2-user/app
cd /home/ec2-user/app
```

#### Set Up .env.production
```bash
# Copy from .env.example and configure production values
cp .env.example .env.production
vim .env.production  # Edit with production values
```

**Required environment variables**:
- Database credentials (`DB_USER`, `DB_PASSWORD`)
- JWT secrets (`JWT_SECRET`, `NUXT_SESSION_PASSWORD`)
- OAuth credentials (`GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`)
- AWS SES configuration
- Domain name

#### Authenticate with GitHub Container Registry
```bash
# Generate GitHub Personal Access Token (PAT)
# GitHub → Settings → Developer settings → Personal access tokens
# Scopes: read:packages

# Login to ghcr.io
echo $GITHUB_TOKEN | docker login ghcr.io -u <github-username> --password-stdin
```

**Alternative: Use GitHub Fine-Grained Token**:
- More secure, repository-scoped
- Permissions: Packages → Read

### 3. First Deployment Test

#### Create Initial Tag
```bash
# On local machine
git tag v1.0.0
git push origin v1.0.0
```

#### Monitor GitHub Actions
- Go to repository → Actions tab
- Watch "CD Pipeline - Deploy to Production" workflow
- Monitor build-and-push job (5-10 minutes)
- Monitor deploy job (2-5 minutes)

#### Verify Deployment
```bash
# SSH to EC2
ssh <ec2-host>
cd /home/<user>/app

# Check containers
docker-compose -f docker-compose.production.yml ps

# Check logs
docker-compose -f docker-compose.production.yml logs -f backend
docker-compose -f docker-compose.production.yml logs -f frontend

# Verify health endpoints
curl http://localhost:8080/health  # Backend
curl http://localhost:3000/api/health  # Frontend
```

#### Test Rollback
```bash
# On EC2 server
export GITHUB_USER=<your-github-username>
./scripts/rollback.sh v1.0.0  # Rollback to v1.0.0
```

---

## Monitoring & Troubleshooting

### Viewing CI/CD Logs

**GitHub Actions UI**:
- Repository → Actions tab
- Click workflow run
- Click job to see logs
- Download artifacts (test results)

**Local GitHub CLI** (optional):
```bash
gh run list  # List recent workflow runs
gh run view <run-id>  # View specific run
gh run watch  # Watch current run
```

### Common CI Failures

#### Backend Tests Fail
- **Cause**: Code changes broke tests
- **Fix**: Run tests locally (`./scripts/test.sh`), fix failing tests

#### Clippy Lints Fail
- **Cause**: Rust code quality issues
- **Fix**: Run `cargo clippy` locally, address warnings

#### Frontend Type Check Fails
- **Cause**: TypeScript type errors
- **Fix**: Run `npx vue-tsc --noEmit` in frontend/, fix type errors

#### Docker Build Fails
- **Cause**: SQLx cache out of date (backend) or TypeScript compilation error (frontend)
- **Fix Backend**: Run `./scripts/prepare-sqlx.sh`, commit `.sqlx/` changes
- **Fix Frontend**: Run `npm run prepare`, fix TypeScript errors

### Common CD Failures

#### Build & Push Fails
- **Cause**: Dockerfile syntax error, missing dependencies
- **Fix**: Test Docker build locally (`docker build -f backend/Dockerfile .`)

#### Deploy Fails (SSH)
- **Cause**: SSH key or host misconfigured
- **Fix**: Verify `EC2_SSH_KEY`, `EC2_HOST`, `EC2_USER` secrets, test SSH manually

#### Health Check Fails
- **Cause**: Service didn't start properly, database migration failed
- **Fix**: SSH to server, check `docker-compose logs`, verify `.env.production`

#### Rollback Fails
- **Cause**: Old version incompatible with current database schema
- **Fix**: Manually run migrations for rollback version, or restore database backup

### Server Monitoring

#### Check Service Status
```bash
docker-compose -f docker-compose.production.yml ps
```

#### View Logs
```bash
# All services
docker-compose -f docker-compose.production.yml logs -f

# Specific service
docker-compose -f docker-compose.production.yml logs -f backend
docker-compose -f docker-compose.production.yml logs -f frontend
```

#### Check Resource Usage
```bash
docker stats  # Real-time resource usage
df -h  # Disk space
free -h  # Memory usage
```

#### Deployment History
```bash
cat deployment-history.log  # Rollback audit trail
```

---

## Performance Considerations

### CI Performance
- **Rust caching**: Swatinem/rust-cache reduces backend CI time (30% faster)
- **Node caching**: setup-node cache reduces frontend CI time (40% faster)
- **Docker cache**: GitHub Actions cache speeds up Docker builds (50% faster)
- **Parallel jobs**: Backend/frontend/Docker jobs run concurrently

**Typical CI Times** (with coverage):
- Backend tests with coverage: 4-7 minutes (cargo-llvm-cov + nextest)
- Backend clippy: 2-4 minutes
- Backend audit: 1-2 minutes
- Frontend tests with coverage: 2.5-3.5 minutes (+30 sec coverage overhead)
- Docker builds: 5-7 minutes
- Codecov upload: ~15 seconds (PR only)
- **Total CI time**: ~9-10 minutes (parallel execution)

### CD Performance
- **Image builds**: 5-10 minutes (cached builds faster)
- **Image push**: 1-2 minutes (depends on image size)
- **Deployment**: 2-5 minutes (pull + restart + migrations)
- **Total CD time**: ~15-20 minutes tag push to live

**Optimization opportunities**:
- Multi-stage Docker builds already minimize image size
- GitHub Actions cache already enabled
- Could add separate staging deployment for faster iteration

### Resource Usage (AWS t3.small - 2GB RAM)
- **CI**: Runs on GitHub runners (no cost to EC2)
- **CD**: Brief CPU spike during deployment, acceptable
- **Docker images**: ~500MB total, well within disk limits
- **Registry storage**: Unlimited public images on ghcr.io

---

## Security Considerations

### GitHub Actions Security

**Secrets Management**:
- SSH keys stored as GitHub Secrets (encrypted at rest)
- Secrets never logged or exposed in CI output
- Repository collaborators cannot access secrets

**Workflow Permissions**:
- Minimal permissions (contents: read, packages: write)
- No write access to repository from workflows
- Environment protection rules for production

**Dependency Security**:
- cargo audit scans Rust dependencies
- npm audit scans Node dependencies
- Fails CI on high-severity vulnerabilities

### Deployment Security

**SSH Security**:
- Dedicated SSH key for GitHub Actions (not personal key)
- ED25519 key algorithm (modern, secure)
- Key scoped to specific user on EC2
- Known hosts verification prevents MITM attacks

**Container Registry Security**:
- GitHub PAT with minimal scope (read:packages only)
- Short-lived tokens preferred
- Images can be private (currently public for open-source)

**Production Server Security**:
- Environment variables in .env.production (not committed)
- Docker secrets not used (would require Swarm/Kubernetes)
- IAM roles for AWS services (not access keys in .env)

### Image Security

**Not implemented** (deferred as discussed):
- Docker image vulnerability scanning (Trivy, Snyk)
- SBOM (Software Bill of Materials) generation
- Image signing

**Rationale**: Low-traffic personal site, cost/benefit doesn't justify complexity. Could add if project grows.

---

## Cost Analysis

### CI/CD Costs

**GitHub Actions** (public repository):
- ✅ Free unlimited minutes for public repos
- ✅ Free unlimited storage for public images on ghcr.io
- ❌ Private repos: 2,000 minutes/month free, then $0.008/minute

**Alternatives**:
- GitLab CI: 400 minutes/month free, then paid
- CircleCI: 6,000 minutes/month free, then paid
- Travis CI: No free tier for private repos

**Current cost**: $0/month

### Deployment Costs

**Existing Infrastructure**:
- AWS EC2 t3.small: ~$15/month (unchanged)
- No additional services needed for CI/CD

**Storage Costs**:
- GitHub Container Registry: Free for public images
- EC2 disk: ~500MB for Docker images (negligible)

**Current cost**: $0/month additional

**Total CI/CD Cost**: $0/month (using existing GitHub Actions free tier)

---

## Future Enhancements

### Deferred (Not Needed Yet)

1. **Blue-Green Deployment**
   - **Why deferred**: Low traffic, brief downtime acceptable
   - **When to implement**: If traffic grows, zero-downtime becomes critical
   - **Cost**: 2x resources (second EC2 instance or container orchestration)

2. **Staging Environment**
   - **Why deferred**: CI provides quality gates, deploy on tag is deliberate
   - **When to implement**: If team grows, want to test integrations before production
   - **Cost**: Additional EC2 instance (~$15/month)

3. **Automated E2E Tests**
   - **Why deferred**: 636 unit/integration tests provide good coverage
   - **When to implement**: If critical user flows need end-to-end validation
   - **Cost**: Playwright/Cypress setup, slower CI

4. **Container Image Scanning**
   - **Why deferred**: Low-traffic personal site, diminishing returns
   - **When to implement**: If handling sensitive user data, compliance requirements
   - **Cost**: Trivy/Snyk integration, potential scan failures to fix

5. **Deployment Notifications**
   - **Why deferred**: GitHub Actions UI sufficient for single developer
   - **When to implement**: If team grows, want Slack/Discord/email alerts
   - **Cost**: Integration setup, potential API costs

### Planned (Near Future)

1. **Branch Protection Rules**
   - Require CI to pass before merge
   - Enforce code review (when team grows)

2. **ESLint in CI**
   - Add ESLint configuration to frontend
   - Enforce consistent code style

3. **Deployment Metrics**
   - Track deployment frequency
   - Track MTTR (Mean Time To Recovery)
   - Track deployment success rate

---

## Related Documentation

- **DEVELOPMENT-WORKFLOW.md**: Daily development workflows, release process
- **IMPLEMENTATION-DEPLOYMENT.md**: Production deployment architecture
- **IMPLEMENTATION-TESTING.md**: Testing strategy and paradigms
- **IMPLEMENTATION-BACKEND.md**: Backend architecture (what CI tests)
- **IMPLEMENTATION-FRONTEND.md**: Frontend architecture (what CI tests)
- **CODING-RULES.md**: Code quality standards (enforced by CI)

---

## Conclusion

This CI/CD implementation provides:

✅ **Automated quality gates**: Tests, linting, security scans on every commit
✅ **Deliberate releases**: Deploy when ready via semantic version tags
✅ **Fast deployments**: Pre-built images, 15-20 minute tag-to-live
✅ **Easy rollback**: Script or re-tag previous version
✅ **Cost-effective**: $0/month (GitHub Actions free tier)
✅ **Professional workflow**: Demonstrates CI/CD understanding for portfolio
✅ **Scalable foundation**: Easy to add staging, notifications, E2E tests later

**Pattern**: Trunk-based development with tag-triggered deployments via GitHub Container Registry

**Philosophy**: Automate quality gates, keep deployment simple and reliable, defer complexity until needed.
