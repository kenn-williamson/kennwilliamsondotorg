# Log Rotation Configuration for KennWilliamson.org

## Overview
This document describes the log rotation and monitoring setup for all Docker containers in the KennWilliamson.org project.

## Log Rotation Strategy

### Docker Log Driver Configuration
All services use the `json-file` log driver with rotation:

```yaml
logging:
  driver: "json-file"
  options:
    max-size: "10m"    # Rotate when log reaches 10MB
    max-file: "3"      # Keep 3 rotated files (30MB total)
```

### Service-Specific Logging

#### Backend (Rust/Actix-web)
- **Log Level**: Controlled by `RUST_LOG` environment variable
- **Default**: `backend=info,actix_web=info`
- **Rate Limiting**: All rate limit violations logged with WARN level
- **Rotation**: 10MB per file, 3 files retained

#### Frontend (Nuxt.js)
- **Log Level**: Controlled by Nuxt logging configuration
- **SSR Logs**: Rate limiting and API calls logged
- **Rotation**: 10MB per file, 3 files retained

#### Nginx
- **Access Logs**: Detailed request logging with timing
- **Error Logs**: Error and warning messages
- **Rotation**: 10MB per file, 3 files retained

#### PostgreSQL
- **Query Logs**: Database queries and errors
- **Connection Logs**: Connection attempts and failures
- **Rotation**: 10MB per file, 3 files retained

#### Redis
- **Command Logs**: Redis commands and errors
- **Memory Logs**: Memory usage and eviction events
- **Rotation**: 10MB per file, 3 files retained

## Log Monitoring Script

### Usage
```bash
# Show log status for all services
./scripts/log-monitor.sh status

# Tail logs for specific service
./scripts/log-monitor.sh tail backend

# Monitor all services in real-time
./scripts/log-monitor.sh monitor -f

# Show log file sizes
./scripts/log-monitor.sh size

# Force log rotation
./scripts/log-monitor.sh rotate

# Clean old logs
./scripts/log-monitor.sh clean
```

### Features
- **Status Monitoring**: Check if containers are running and log activity
- **Real-time Monitoring**: Follow logs from multiple services simultaneously
- **Size Management**: Monitor log file sizes and disk usage
- **Rotation Control**: Force log rotation when needed
- **Cleanup**: Remove old logs and unused Docker resources

## Rate Limiting Logs

### Backend Rate Limiting
- **Violations**: Logged with WARN level
- **Format**: `Rate limit exceeded for {identifier} on {endpoint}: {count} requests/hour`
- **Monitoring**: Track patterns of abuse and adjust limits

### Frontend SSR Rate Limiting
- **Violations**: Logged to console with warning level
- **Format**: `SSR Rate limit exceeded for {identifier} on {endpoint}: {count} requests/hour`
- **Monitoring**: Track SSR-specific abuse patterns

## Log Analysis

### Key Metrics to Monitor
1. **Rate Limit Violations**: Frequency and patterns
2. **Error Rates**: 4xx and 5xx response rates
3. **Response Times**: API performance degradation
4. **Resource Usage**: Memory and CPU spikes
5. **Database Performance**: Query times and connection issues

### Alert Conditions
- **High Error Rate**: >5% 5xx responses
- **Rate Limit Abuse**: >10 violations per hour from same IP
- **Slow Responses**: >2s average response time
- **Resource Exhaustion**: >80% memory usage
- **Database Issues**: Connection failures or slow queries

## Production Considerations

### Log Retention
- **Development**: 3 files × 10MB = 30MB per service
- **Production**: Consider longer retention for audit trails
- **Compliance**: Adjust retention based on regulatory requirements

### External Logging
For production, consider:
- **ELK Stack**: Elasticsearch, Logstash, Kibana
- **Fluentd**: Log aggregation and forwarding
- **Cloud Logging**: AWS CloudWatch, Google Cloud Logging
- **Splunk**: Enterprise log management

### Security
- **Sensitive Data**: Never log passwords, tokens, or PII
- **Access Control**: Restrict log file access
- **Encryption**: Encrypt logs in transit and at rest
- **Audit Trails**: Maintain immutable audit logs

## Troubleshooting

### Common Issues
1. **Logs Not Rotating**: Check Docker log driver configuration
2. **High Disk Usage**: Run `./scripts/log-monitor.sh clean`
3. **Missing Logs**: Verify container is running and logging is enabled
4. **Performance Impact**: Monitor log I/O impact on services

### Debug Commands
```bash
# Check Docker log configuration
docker inspect <container_name> | grep -A 10 LogConfig

# View raw log files
docker logs <container_name> --details

# Monitor disk usage
docker system df

# Check log rotation
ls -la /var/lib/docker/containers/*/
```

## Maintenance Schedule

### Daily
- Monitor log status: `./scripts/log-monitor.sh status`
- Check for errors in logs
- Monitor rate limiting violations

### Weekly
- Review log sizes: `./scripts/log-monitor.sh size`
- Clean old logs if needed: `./scripts/log-monitor.sh clean`
- Analyze rate limiting patterns

### Monthly
- Review log retention policies
- Update log monitoring scripts
- Performance analysis and optimization

---

*This log rotation setup ensures reliable logging while preventing disk space issues and maintaining system performance.*
