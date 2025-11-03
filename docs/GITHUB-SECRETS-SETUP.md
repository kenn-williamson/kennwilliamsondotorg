# GitHub Secrets Setup Guide

This guide walks through configuring GitHub repository secrets required for the automated CD pipeline.

## Required Secrets

The deployment workflow requires three secrets to access your EC2 instance:

| Secret Name | Description | Example Value |
|-------------|-------------|---------------|
| `EC2_SSH_KEY` | Private SSH key for authentication | `-----BEGIN OPENSSH PRIVATE KEY-----\n...` |
| `EC2_HOST` | EC2 instance hostname or IP address | `ec2-12-34-56-78.compute-1.amazonaws.com` or `12.34.56.78` |
| `EC2_USER` | SSH username on EC2 instance | `ubuntu` |

---

## Step 1: Get Your SSH Private Key

You need the **private key** that corresponds to the public key configured on your EC2 instance.

### Option A: Using Existing Key Pair

If you already SSH into your EC2 instance, you have the private key locally:

```bash
# Find your SSH key (usually in ~/.ssh/)
ls -la ~/.ssh/

# Common key names:
# - id_rsa (older RSA keys)
# - id_ed25519 (modern Ed25519 keys)
# - your-ec2-keypair.pem (AWS-generated keys)

# Display the private key content
cat ~/.ssh/your-key-name
```

Copy the **entire output** including the header and footer:
```
-----BEGIN OPENSSH PRIVATE KEY-----
... (many lines of base64-encoded data) ...
-----END OPENSSH PRIVATE KEY-----
```

### Option B: Creating New Key Pair (if needed)

If you need to create a new SSH key pair:

```bash
# Generate Ed25519 key pair (recommended)
ssh-keygen -t ed25519 -C "github-actions-deploy" -f ~/.ssh/github_deploy_key

# Or RSA if Ed25519 not supported
ssh-keygen -t rsa -b 4096 -C "github-actions-deploy" -f ~/.ssh/github_deploy_key
```

**Then add the public key to your EC2 instance:**

```bash
# Copy public key content
cat ~/.ssh/github_deploy_key.pub

# SSH into your EC2 instance
ssh ubuntu@your-ec2-host

# On EC2: Add public key to authorized_keys
echo "YOUR_PUBLIC_KEY_HERE" >> ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys

# Test the new key from your local machine
ssh -i ~/.ssh/github_deploy_key ubuntu@your-ec2-host
```

---

## Step 2: Get Your EC2 Host

You need your EC2 instance's public hostname or IP address.

### Option A: AWS Console

1. Go to EC2 Dashboard → Instances
2. Select your instance
3. Copy either:
   - **Public IPv4 address**: `12.34.56.78`
   - **Public IPv4 DNS**: `ec2-12-34-56-78.compute-1.amazonaws.com`

### Option B: AWS CLI

```bash
# List all running instances with their public IPs
aws ec2 describe-instances \
  --filters "Name=instance-state-name,Values=running" \
  --query "Reservations[*].Instances[*].[InstanceId,PublicIpAddress,PublicDnsName]" \
  --output table
```

### Option C: Check DNS Records

If you have a domain pointing to your EC2 instance:

```bash
# Look up the DNS record for kennwilliamson.org
dig kennwilliamson.org +short
nslookup kennwilliamson.org
```

**Note**: You can use either the IP address or DNS name. DNS name is preferred if using Elastic IP (won't change).

---

## Step 3: Confirm SSH Username

For your EC2 instance running Ubuntu, the username is `ubuntu`.

**Common usernames by AMI:**
- Ubuntu AMI: `ubuntu`
- Amazon Linux 2: `ec2-user`
- Debian AMI: `admin`
- RHEL AMI: `ec2-user`

**Test your username:**
```bash
ssh -i ~/.ssh/your-key ubuntu@your-ec2-host
```

---

## Step 4: Add Secrets to GitHub

### Navigate to Repository Secrets

1. Go to your GitHub repository: `https://github.com/YOUR_USERNAME/kennwilliamsondotorg`
2. Click **Settings** tab (top navigation)
3. In left sidebar, click **Secrets and variables** → **Actions**
4. Click **New repository secret** button

### Add EC2_SSH_KEY Secret

1. Click **New repository secret**
2. Name: `EC2_SSH_KEY`
3. Value: Paste the **entire private key** including headers
   ```
   -----BEGIN OPENSSH PRIVATE KEY-----
   ... (your private key content) ...
   -----END OPENSSH PRIVATE KEY-----
   ```
4. Click **Add secret**

**Important**:
- Include the `-----BEGIN` and `-----END` lines
- Preserve all newlines (don't condense to single line)
- No extra spaces before/after

### Add EC2_HOST Secret

1. Click **New repository secret**
2. Name: `EC2_HOST`
3. Value: Your EC2 hostname or IP
   - Example DNS: `ec2-12-34-56-78.compute-1.amazonaws.com`
   - Example IP: `12.34.56.78`
4. Click **Add secret**

### Add EC2_USER Secret

1. Click **New repository secret**
2. Name: `EC2_USER`
3. Value: `ubuntu`
4. Click **Add secret**

---

## Step 5: Verify Secrets Configuration

After adding all three secrets, you should see them listed (values hidden):

```
EC2_SSH_KEY     Updated 1 minute ago
EC2_HOST        Updated 1 minute ago
EC2_USER        Updated 1 minute ago
```

---

## Step 6: Test SSH Connection

Before triggering a deployment, test that GitHub Actions can connect:

### Create Test Workflow (Optional)

Create `.github/workflows/test-ssh.yml`:

```yaml
name: Test SSH Connection
on:
  workflow_dispatch:  # Manual trigger only

jobs:
  test-connection:
    runs-on: ubuntu-latest
    steps:
      - name: Test SSH
        run: |
          mkdir -p ~/.ssh
          echo "${{ secrets.EC2_SSH_KEY }}" > ~/.ssh/test_key
          chmod 600 ~/.ssh/test_key
          ssh-keyscan -H ${{ secrets.EC2_HOST }} >> ~/.ssh/known_hosts

          ssh -i ~/.ssh/test_key -o ConnectTimeout=10 ${{ secrets.EC2_USER }}@${{ secrets.EC2_HOST }} \
            "echo 'SSH connection successful!'; hostname; uptime"

          rm -f ~/.ssh/test_key
```

**Run the test:**
1. Go to **Actions** tab
2. Select **Test SSH Connection** workflow
3. Click **Run workflow** → **Run workflow**
4. Check the logs for "SSH connection successful!"

---

## Troubleshooting

### Error: "Permission denied (publickey)"

**Cause**: Private key doesn't match public key on server, or key permissions incorrect.

**Fix**:
1. Verify the private key is correct (matches public key on EC2)
2. Check EC2 instance's `~/.ssh/authorized_keys` contains the public key
3. Ensure key format is correct (include BEGIN/END lines)

### Error: "Host key verification failed"

**Cause**: GitHub Actions doesn't recognize the server's SSH fingerprint.

**Fix**: The workflow uses `ssh-keyscan` to automatically add the host. This should not occur unless EC2_HOST is wrong.

### Error: "Connection timeout"

**Cause**: Cannot reach EC2 instance on port 22.

**Fix**:
1. Verify security group allows inbound SSH (port 22) from GitHub Actions IPs
2. Consider allowing all IPs: `0.0.0.0/0` for port 22 (or restrict to GitHub's IP ranges)
3. Check EC2 instance is running and has public IP

### Error: "EC2_SSH_KEY contains invalid characters"

**Cause**: Key was corrupted during copy/paste.

**Fix**:
1. Re-copy the private key carefully
2. Use `cat ~/.ssh/your-key | pbcopy` (macOS) or `cat ~/.ssh/your-key | xclip` (Linux)
3. Ensure newlines are preserved

---

## Security Best Practices

### 1. Use Deployment-Specific Key Pair
Create a separate SSH key pair just for GitHub Actions deployment. Don't reuse your personal SSH key.

### 2. Restrict Key Permissions on Server
```bash
# On EC2 instance
chmod 700 ~/.ssh
chmod 600 ~/.ssh/authorized_keys
```

### 3. Use GitHub Environments (Optional)
For additional protection, use GitHub Environments with required reviewers:

1. Go to Settings → Environments
2. Create "production" environment
3. Add protection rules (require reviewers, wait timer)
4. The workflow already uses `environment: production`

### 4. Rotate Keys Periodically
Generate new key pairs every 6-12 months:
1. Create new key pair
2. Add new public key to EC2
3. Update `EC2_SSH_KEY` secret
4. Test deployment
5. Remove old public key from EC2

### 5. Monitor Deployment Logs
Check GitHub Actions logs regularly for suspicious activity or failed deployments.

---

## Next Steps

After configuring secrets:

1. **Verify `.env.production` exists on server** at `/opt/kennwilliamson/kennwilliamsondotorg/.env.production`
2. **Ensure server has Docker and Docker Compose installed**
3. **Authenticate server with GitHub Container Registry** (if using private images)
4. **Create and push a test tag** to trigger deployment

See [IMPLEMENTATION-DEPLOYMENT.md](../IMPLEMENTATION-DEPLOYMENT.md) for full deployment instructions.
