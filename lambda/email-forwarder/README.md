# Email Forwarder Lambda Function

AWS Lambda function that forwards emails received by SES to a specified email address.

**Runtime**: Node.js 24.x (AWS Lambda `nodejs24.x`)

## How It Works

1. Email arrives at `*@kennwilliamson.org`
2. SES receives email via MX records
3. SES stores email in S3 bucket
4. S3 PUT event triggers this Lambda function
5. Lambda reads email from S3
6. Lambda modifies headers and forwards to configured address
7. Original email preserved with forwarding metadata

## Environment Variables

Required:
- `FORWARD_TO`: Destination email address (e.g., `kenn@seqtek.com`)

Optional:
- `FROM_EMAIL`: Sender address for forwarded emails (default: `forwarded@kennwilliamson.org`)
- `AWS_REGION`: AWS region (default: `us-east-1`)

## IAM Permissions Required

The Lambda execution role needs:

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "s3:GetObject"
      ],
      "Resource": "arn:aws:s3:::kennwilliamson-incoming-emails/*"
    },
    {
      "Effect": "Allow",
      "Action": [
        "ses:SendRawEmail"
      ],
      "Resource": "*"
    },
    {
      "Effect": "Allow",
      "Action": [
        "logs:CreateLogGroup",
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Resource": "arn:aws:logs:*:*:*"
    }
  ]
}
```

## Deployment

### First Time Setup

1. Install dependencies:
   ```bash
   cd lambda/email-forwarder
   npm install
   ```

2. Package function:
   ```bash
   zip -r function.zip index.js node_modules/ package.json
   ```

3. Create Lambda function (see setup instructions in project docs)

### Updates

After modifying the code:

```bash
cd lambda/email-forwarder
npm install  # if dependencies changed
npm run package
aws lambda update-function-code \
  --function-name email-forwarder \
  --zip-file fileb://function.zip \
  --region us-east-1
```

### Runtime Update

To update the Node.js runtime (e.g., to Node.js 24.x LTS):

```bash
aws lambda update-function-configuration \
  --function-name email-forwarder \
  --runtime nodejs24.x \
  --region us-east-1
```

## Testing

Send a test email to any address at kennwilliamson.org:
```bash
# Using AWS SES simulator
aws ses send-email \
  --from test@verified-domain.com \
  --to test@kennwilliamson.org \
  --subject "Test Forward" \
  --text "This should be forwarded" \
  --region us-east-1
```

Check CloudWatch Logs for function execution:
```bash
aws logs tail /aws/lambda/email-forwarder --follow --region us-east-1
```

## Email Modifications

The forwarder makes these changes:

**Subject Line:**
- Original: `Question about your project`
- Forwarded: `Question about your project from john@example.com`

**From Address:**
- Changed to: `forwarded@kennwilliamson.org`
- Avoids SPF/DKIM authentication issues

**Headers Added:**
- `X-Original-To`: Original recipient (e.g., `info@kennwilliamson.org`)
- `X-Original-From`: Original sender (e.g., `john@example.com`)
- `X-Forwarded-By`: Identifies this Lambda function

These headers allow you to create email filters based on the original recipient address.

## Spam and Virus Filtering

The forwarder automatically filters spam and viruses:

**SES Scanning:**
- All incoming emails are scanned by AWS SES
- Verdicts added to email headers: `X-SES-Spam-Verdict` and `X-SES-Virus-Verdict`

**Lambda Filtering:**
- Emails marked as spam (`FAIL` verdict) are **NOT** forwarded
- Emails with viruses (`FAIL` verdict) are **NOT** forwarded
- Blocked emails are logged in CloudWatch for review
- Clean emails are forwarded normally

**Verdicts:**
- `PASS` - Clean email, will be forwarded
- `FAIL` - Spam/virus detected, will NOT be forwarded
- `GRAY` - Uncertain, will be forwarded (rare)
- `PROCESSING_FAILED` - Scan failed, will be forwarded

## Limitations

- Maximum email size: 10 MB (SES limit)
- DKIM signatures are removed (would be invalid after modification)
- Emails stored in S3 for 30 days (configurable lifecycle policy)
- Spam filtering is based on SES verdicts only (not customizable)
