# Logging Implementation

## Overview
Logging strategy decisions for development and production environments.

## Logging Philosophy

### What We Log
**Decision**: Structured logging at appropriate levels

**Why:**
- INFO: Business events, user actions
- WARN: Rate limiting, recoverable issues
- ERROR: Failures, exceptions
- DEBUG: Development diagnostics

**What we don't log:**
- Passwords, tokens, PII
- Excessive debug in production
- Request bodies with sensitive data

## Architecture Decisions

### Docker Log Driver
**Decision**: JSON file driver with rotation

**Configuration:**
- 10MB per file
- 3 rotated files (30MB total per service)
- Automatic rotation

**Why:**
- Prevents disk exhaustion
- JSON format for parsing
- Standard Docker tooling works
- No additional log daemon needed

**Trade-offs:**
- 30MB limit might be low for production
- Could upgrade to ELK stack later
- Worth it: Simple, works out-of-box

### Rate Limiting Logs
**Decision**: Log all rate limit violations at WARN level

**Why:**
- Track abuse patterns
- Adjust limits based on data
- Security monitoring

**Format:**
```
WARN: Rate limit exceeded for {id} on {endpoint}: {count} requests/hour
```

### Service-Specific Logging

**Backend (Rust):**
- RUST_LOG environment variable
- actix_web logs for HTTP
- Structured with context

**Frontend (Nuxt):**
- SSR logs for server-side
- Browser console for client-side
- Rate limiting violations logged

**Nginx:**
- Access logs with timing
- Error logs for failures
- Upstream failures logged

**PostgreSQL:**
- Query logs in development
- Connection logs always
- Error logs always

## Log Monitoring

**Development:**
- `./scripts/dev-logs.sh [service]`
- Real-time tailing
- Service-specific viewing

**Production:**
- `./scripts/log-monitor.sh status`
- Size monitoring
- Rotation management

## Production Considerations

### External Logging (Future)
**Options:**
- ELK Stack (Elasticsearch + Logstash + Kibana)
- Cloud logging (AWS CloudWatch)
- Fluentd for aggregation

**When to add:**
- Multiple server instances
- Compliance requirements
- Need for log search/analysis
- Alert automation

**Current approach sufficient for:**
- Single-instance deployment
- Basic monitoring needs
- Development workflows

### Security
**Requirements:**
- Never log sensitive data
- Restrict log file access
- Audit trail for security events

**Implementation:**
- Sanitize before logging
- File permissions on logs
- Security events at WARN/ERROR
