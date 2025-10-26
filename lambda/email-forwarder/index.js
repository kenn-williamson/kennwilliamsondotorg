/**
 * AWS Lambda Email Forwarder
 *
 * Forwards emails received by SES to a specified email address.
 * Triggered by S3 PUT events when SES stores incoming emails.
 *
 * Environment Variables:
 * - FORWARD_TO: Email address to forward to (e.g., kenn@seqtek.com)
 * - FROM_EMAIL: Email to use as sender (default: forwarded@kennwilliamson.org)
 *
 * IAM Permissions Required:
 * - s3:GetObject (to read email from S3)
 * - ses:SendRawEmail (to forward email)
 */

const { S3Client, GetObjectCommand, CopyObjectCommand, DeleteObjectCommand } = require('@aws-sdk/client-s3');
const { SESClient, SendRawEmailCommand } = require('@aws-sdk/client-ses');

const s3Client = new S3Client({ region: process.env.AWS_REGION || 'us-east-1' });
const sesClient = new SESClient({ region: process.env.AWS_REGION || 'us-east-1' });

/**
 * Lambda handler function
 */
exports.handler = async (event) => {
    console.log('Email forwarding triggered:', JSON.stringify(event, null, 2));

    const forwardTo = process.env.FORWARD_TO;
    const fromEmail = process.env.FROM_EMAIL || 'forwarded@kennwilliamson.org';

    if (!forwardTo) {
        throw new Error('FORWARD_TO environment variable not set');
    }

    try {
        // Process each S3 record (SES writes one email per record)
        for (const record of event.Records) {
            const bucket = record.s3.bucket.name;
            const key = decodeURIComponent(record.s3.object.key.replace(/\+/g, ' '));

            console.log(`Processing email from s3://${bucket}/${key}`);

            // Get the email from S3
            const getObjectCommand = new GetObjectCommand({
                Bucket: bucket,
                Key: key,
            });

            const s3Response = await s3Client.send(getObjectCommand);
            const emailData = await streamToString(s3Response.Body);

            // Parse email to extract headers
            const originalTo = extractHeader(emailData, 'To');
            const originalFrom = extractHeader(emailData, 'From');
            const subject = extractHeader(emailData, 'Subject');

            // Check SES spam/virus verdicts
            const spamVerdict = extractHeader(emailData, 'X-SES-Spam-Verdict');
            const virusVerdict = extractHeader(emailData, 'X-SES-Virus-Verdict');

            console.log(`Email verdicts: Spam=${spamVerdict}, Virus=${virusVerdict}`);

            // Determine email verdict and target directory
            let targetDir = 'emails/forwarded/';
            let shouldForward = true;

            if (spamVerdict === 'FAIL') {
                console.log(`SPAM DETECTED - Not forwarding: From=${originalFrom}, Subject=${subject}`);
                targetDir = 'emails/spam/';
                shouldForward = false;
            } else if (virusVerdict === 'FAIL') {
                console.log(`VIRUS DETECTED - Not forwarding: From=${originalFrom}, Subject=${subject}`);
                targetDir = 'emails/virus/';
                shouldForward = false;
            } else {
                console.log(`Forwarding clean email: From=${originalFrom}, To=${originalTo}, Subject=${subject}`);
            }

            // Move email to appropriate subdirectory
            const fileName = key.split('/').pop(); // Get filename from path
            const newKey = targetDir + fileName;

            await s3Client.send(new CopyObjectCommand({
                Bucket: bucket,
                CopySource: `${bucket}/${key}`,
                Key: newKey,
            }));

            await s3Client.send(new DeleteObjectCommand({
                Bucket: bucket,
                Key: key,
            }));

            console.log(`Email moved to ${newKey}`);

            // Skip forwarding if spam/virus
            if (!shouldForward) {
                continue;
            }

            // Modify email headers for forwarding
            const modifiedEmail = modifyEmailHeaders(emailData, {
                forwardTo,
                fromEmail,
                originalTo,
                originalFrom,
            });

            // Send the modified email via SES
            const sendCommand = new SendRawEmailCommand({
                RawMessage: {
                    Data: Buffer.from(modifiedEmail),
                },
                Source: fromEmail,
                Destinations: [forwardTo],
            });

            const sesResponse = await sesClient.send(sendCommand);
            console.log(`Email forwarded successfully. MessageId: ${sesResponse.MessageId}`);
        }

        return {
            statusCode: 200,
            body: 'Email(s) forwarded successfully',
        };
    } catch (error) {
        console.error('Error forwarding email:', error);
        throw error;
    }
};

/**
 * Convert stream to string
 */
async function streamToString(stream) {
    const chunks = [];
    for await (const chunk of stream) {
        chunks.push(chunk);
    }
    return Buffer.concat(chunks).toString('utf-8');
}

/**
 * Extract email header value
 */
function extractHeader(email, headerName) {
    const regex = new RegExp(`^${headerName}:\\s*(.*)$`, 'mi');
    const match = email.match(regex);
    return match ? match[1].trim() : '';
}

/**
 * Modify email headers for forwarding
 */
function modifyEmailHeaders(emailData, { forwardTo, fromEmail, originalTo, originalFrom }) {
    // Split headers and body
    const parts = emailData.split(/\r?\n\r?\n/);
    let headers = parts[0];
    const body = parts.slice(1).join('\n\n');

    // Extract original subject
    const originalSubject = extractHeader(emailData, 'Subject');

    // Remove headers that would cause issues with forwarding
    headers = headers.replace(/^Return-Path:.*$/mi, '');
    headers = headers.replace(/^Sender:.*$/mi, '');
    headers = headers.replace(/^Message-ID:.*$/mi, '');
    headers = headers.replace(/^DKIM-Signature:.*$/mi, '');

    // Replace To and From headers
    headers = headers.replace(/^To:.*$/mi, `To: ${forwardTo}`);
    headers = headers.replace(/^From:.*$/mi, `From: ${fromEmail}`);

    // Modify subject to include original sender
    const newSubject = `${originalSubject} from ${originalFrom}`;
    headers = headers.replace(/^Subject:.*$/mi, `Subject: ${newSubject}`);

    // Add forwarding information headers (for filtering/debugging)
    headers += `\nX-Original-To: ${originalTo}`;
    headers += `\nX-Original-From: ${originalFrom}`;
    headers += `\nX-Forwarded-By: AWS Lambda Email Forwarder`;

    // Reconstruct email
    return `${headers}\n\n${body}`;
}
