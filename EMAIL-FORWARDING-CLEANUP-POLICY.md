# Email Forwarding Cleanup Policy

## Automatic Email Organization

Emails are automatically organized by AWS Lambda based on SES spam/virus scanning:

```
s3://kennwilliamson-incoming-emails/emails/
  ├── forwarded/  - Clean emails (forwarded to kenn@seqtek.com)
  ├── spam/       - Spam emails (blocked, not forwarded)
  └── virus/      - Virus emails (blocked, not forwarded)
```

## Automatic Cleanup (S3 Lifecycle)

AWS S3 automatically deletes old emails - **zero maintenance required:**

- **Clean emails**: Deleted after **30 days** (already in your inbox)
- **Spam/virus**: Deleted after **90 days** (allows review of false positives)

**Cost:** ~$0.00-0.01/month (well within free tier)

## Managing the Policy

### View current policy:
```bash
aws s3api get-bucket-lifecycle-configuration \
  --bucket kennwilliamson-incoming-emails
```

### Modify retention periods:
```bash
# Edit lifecycle-policy.json and apply:
aws s3api put-bucket-lifecycle-configuration \
  --bucket kennwilliamson-incoming-emails \
  --lifecycle-configuration file://lifecycle-policy.json
```

### Review blocked emails:
```bash
# List spam
aws s3 ls s3://kennwilliamson-incoming-emails/emails/spam/

# Download to review
aws s3 cp s3://kennwilliamson-incoming-emails/emails/spam/MESSAGE_ID /tmp/
```

See `lambda/email-forwarder/README.md` for full Lambda documentation.
