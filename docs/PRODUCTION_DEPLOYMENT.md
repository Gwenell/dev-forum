# Developer Forum: Production Deployment Guide

This guide provides detailed instructions for deploying the Developer Forum application in a production environment on a Debian-based server.

## Prerequisites

- Debian-based server (Debian 11+ or Ubuntu 20.04+)
- Root access or sudo privileges
- Domain name with DNS properly configured
- Basic knowledge of Linux server administration

## System Preparation

### Update System Packages

```bash
sudo apt update
sudo apt upgrade -y
```

### Set Up Firewall

Install and configure UFW (Uncomplicated Firewall):

```bash
sudo apt install -y ufw
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow ssh
sudo ufw allow 80
sudo ufw allow 443
sudo ufw enable
```

### Install Required Dependencies

```bash
# Install common utilities
sudo apt install -y curl wget git build-essential

# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Install Node.js and npm (using NodeSource repository for latest stable version)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs

# Install MariaDB client and development libraries
sudo apt install -y default-mysql-client default-libmysqlclient-dev

# Install ClamAV for virus scanning
sudo apt install -y clamav clamav-daemon
sudo systemctl enable clamav-daemon
sudo systemctl start clamav-daemon

# Install Nginx for reverse proxy
sudo apt install -y nginx
```

## Database Setup

### Install MariaDB Server (if not already installed)

```bash
sudo apt install -y mariadb-server
sudo systemctl enable mariadb
sudo systemctl start mariadb
```

### Secure MariaDB Installation

```bash
sudo mysql_secure_installation
```

Follow the prompts to set a root password, remove anonymous users, disallow root login remotely, remove test database, and reload privileges.

### Create Database and User

```bash
sudo mysql -u root -p
```

In the MySQL console:

```sql
CREATE DATABASE dev_forum CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
CREATE USER 'dev_forum_user'@'localhost' IDENTIFIED BY 'StrongPassword123!';
GRANT ALL PRIVILEGES ON dev_forum.* TO 'dev_forum_user'@'localhost';
FLUSH PRIVILEGES;
EXIT;
```

Replace `'StrongPassword123!'` with a secure password.

## Application Setup

### Create Application User

Create a dedicated user for running the application:

```bash
sudo adduser --system --group --shell /bin/bash dev_forum
```

### Clone Repository

```bash
sudo mkdir -p /var/www
sudo chown dev_forum:dev_forum /var/www
sudo -u dev_forum git clone [repository_url] /var/www/dev-forum
```

Replace `[repository_url]` with your Git repository URL.

### Configure Backend

Create environment file:

```bash
sudo -u dev_forum touch /var/www/dev-forum/backend/.env
sudo nano /var/www/dev-forum/backend/.env
```

Add the following content:

```
DATABASE_URL=mysql://dev_forum_user:StrongPassword123!@localhost/dev_forum
JWT_SECRET=generate_a_strong_random_secret_here
PORT=8000
FRONTEND_URL=https://yourdomain.com
MAX_UPLOAD_SIZE=20971520
```

Replace `StrongPassword123!` with the database password you set earlier, and `generate_a_strong_random_secret_here` with a strong random string. You can generate one with:

```bash
openssl rand -base64 32
```

Replace `yourdomain.com` with your actual domain.

### Configure Frontend

Create environment file:

```bash
sudo -u dev_forum touch /var/www/dev-forum/frontend/.env
sudo nano /var/www/dev-forum/frontend/.env
```

Add the following content:

```
VITE_API_URL=https://yourdomain.com/api
VITE_WS_URL=wss://yourdomain.com/ws
```

Replace `yourdomain.com` with your actual domain.

### Build the Backend

```bash
cd /var/www/dev-forum/backend
sudo -u dev_forum cargo build --release
```

### Build the Frontend

```bash
cd /var/www/dev-forum/frontend
sudo -u dev_forum npm install
sudo -u dev_forum npm run build
```

## Setting Up Services

### Backend Service

Create a systemd service for the backend:

```bash
sudo nano /etc/systemd/system/dev-forum-backend.service
```

Add the following content:

```
[Unit]
Description=Developer Forum Backend
After=network.target
After=mariadb.service

[Service]
Type=simple
User=dev_forum
Group=dev_forum
WorkingDirectory=/var/www/dev-forum/backend
ExecStart=/var/www/dev-forum/backend/target/release/dev-forum-backend
Restart=on-failure
# Environment variables from .env file
EnvironmentFile=/var/www/dev-forum/backend/.env

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable dev-forum-backend
sudo systemctl start dev-forum-backend
```

### Frontend Serving

We'll use Nginx to serve the frontend static files and act as a reverse proxy for the backend.

## Nginx Configuration

### Create Nginx Configuration

```bash
sudo nano /etc/nginx/sites-available/dev-forum
```

Add the following content:

```nginx
server {
    listen 80;
    server_name yourdomain.com www.yourdomain.com;
    
    # Redirect HTTP to HTTPS
    location / {
        return 301 https://$host$request_uri;
    }
}

server {
    listen 443 ssl http2;
    server_name yourdomain.com www.yourdomain.com;
    
    # SSL configuration
    ssl_certificate /etc/letsencrypt/live/yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/yourdomain.com/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_prefer_server_ciphers on;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 1d;
    ssl_session_tickets off;
    
    # HSTS (optional, but recommended)
    add_header Strict-Transport-Security "max-age=63072000; includeSubDomains; preload" always;
    
    # Other security headers
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    
    # Frontend static files
    root /var/www/dev-forum/frontend/build;
    index index.html;
    
    # API endpoints proxy
    location /api/ {
        proxy_pass http://localhost:8000/api/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
    
    # WebSocket proxy
    location /ws/ {
        proxy_pass http://localhost:8000/ws/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "Upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
    
    # Handle client-side routing
    location / {
        try_files $uri $uri/ /index.html;
    }
    
    # File upload limit
    client_max_body_size 100M;
    
    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
        expires 30d;
        add_header Cache-Control "public, no-transform";
    }
}
```

Replace `yourdomain.com` with your actual domain.

### Enable the Configuration

```bash
sudo ln -s /etc/nginx/sites-available/dev-forum /etc/nginx/sites-enabled/
sudo rm -f /etc/nginx/sites-enabled/default
sudo nginx -t
sudo systemctl restart nginx
```

## SSL Certificate with Let's Encrypt

### Install Certbot

```bash
sudo apt install -y certbot python3-certbot-nginx
```

### Obtain SSL Certificate

```bash
sudo certbot --nginx -d yourdomain.com -d www.yourdomain.com
```

Follow the prompts to complete the SSL certificate installation.

## File Upload Directory

Create and secure a directory for file uploads:

```bash
sudo mkdir -p /var/www/dev-forum/uploads
sudo chown dev_forum:dev_forum /var/www/dev-forum/uploads
sudo chmod 750 /var/www/dev-forum/uploads
```

## Automated Backups

### Create Backup Script

```bash
sudo nano /opt/backup-dev-forum.sh
```

Add the following content:

```bash
#!/bin/bash

# Configuration
TIMESTAMP=$(date +"%Y%m%d%H%M%S")
BACKUP_DIR="/var/backups/dev-forum"
DB_USER="dev_forum_user"
DB_PASS="StrongPassword123!"
DB_NAME="dev_forum"
APP_DIR="/var/www/dev-forum"
UPLOADS_DIR="/var/www/dev-forum/uploads"

# Create backup directory if it doesn't exist
mkdir -p "$BACKUP_DIR"

# Backup database
mysqldump --user="$DB_USER" --password="$DB_PASS" "$DB_NAME" | gzip > "$BACKUP_DIR/db_${TIMESTAMP}.sql.gz"

# Backup uploads
tar -czf "$BACKUP_DIR/uploads_${TIMESTAMP}.tar.gz" -C "$(dirname "$UPLOADS_DIR")" "$(basename "$UPLOADS_DIR")"

# Backup application code
tar -czf "$BACKUP_DIR/app_${TIMESTAMP}.tar.gz" -C "$(dirname "$APP_DIR")" "$(basename "$APP_DIR")" --exclude="$APP_DIR/node_modules" --exclude="$APP_DIR/target"

# Remove backups older than 7 days
find "$BACKUP_DIR" -type f -name "*.gz" -mtime +7 -delete

# Set proper permissions
chmod 600 "$BACKUP_DIR"/*.gz
```

Make the script executable:

```bash
sudo chmod +x /opt/backup-dev-forum.sh
```

### Schedule Automatic Backups

```bash
sudo crontab -e
```

Add the following line to run the backup daily at 2 AM:

```
0 2 * * * /opt/backup-dev-forum.sh
```

## Log Rotation

Create a logrotate configuration:

```bash
sudo nano /etc/logrotate.d/dev-forum
```

Add the following content:

```
/var/log/dev-forum-backend.log {
    daily
    rotate 14
    compress
    delaycompress
    missingok
    notifempty
    create 0640 dev_forum dev_forum
    postrotate
        systemctl reload dev-forum-backend
    endscript
}
```

## Monitoring Setup

### Install Monitoring Tools

```bash
sudo apt install -y prometheus node-exporter
```

### Configure Prometheus for Basic Monitoring

```bash
sudo nano /etc/prometheus/prometheus.yml
```

Add the following job:

```yaml
  - job_name: 'node_exporter'
    static_configs:
      - targets: ['localhost:9100']
```

Restart Prometheus:

```bash
sudo systemctl restart prometheus
```

## Security Enhancements

### Set Up Fail2ban

```bash
sudo apt install -y fail2ban
sudo cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local
sudo nano /etc/fail2ban/jail.local
```

Configure Fail2ban for SSH and Nginx:

```
[sshd]
enabled = true
port = ssh
filter = sshd
logpath = /var/log/auth.log
maxretry = 5
findtime = 300
bantime = 3600

[nginx-http-auth]
enabled = true
filter = nginx-http-auth
port = http,https
logpath = /var/log/nginx/error.log
maxretry = 5
```

Restart Fail2ban:

```bash
sudo systemctl restart fail2ban
```

## Final Steps

### Verify Deployment

1. Check that all services are running:

   ```bash
   sudo systemctl status dev-forum-backend
   sudo systemctl status nginx
   sudo systemctl status mariadb
   sudo systemctl status clamav-daemon
   ```

2. Open your domain in a browser and verify that the application loads correctly.

3. Test user registration and login functionality.

4. Verify that file uploads and malware scanning work properly.

5. Test real-time chat and WebRTC streaming.

### Performance Tuning

For improved performance on production servers:

1. Configure Nginx worker processes based on available CPU cores:

   ```bash
   sudo nano /etc/nginx/nginx.conf
   ```

   Set `worker_processes` to the number of CPU cores (or `auto`).

2. Optimize MariaDB for your server's resources:

   ```bash
   sudo nano /etc/mysql/mariadb.conf.d/50-server.cnf
   ```

   Adjust settings like `innodb_buffer_pool_size` based on available RAM.

## Troubleshooting

### Common Issues

1. **Backend won't start**:
   - Check logs: `journalctl -u dev-forum-backend`
   - Verify database connection
   - Ensure port 8000 is not in use

2. **Nginx configuration errors**:
   - Test configuration: `sudo nginx -t`
   - Check logs: `sudo tail -f /var/log/nginx/error.log`

3. **SSL certificate issues**:
   - Verify certificate paths
   - Run Certbot renewal test: `sudo certbot renew --dry-run`

4. **File upload problems**:
   - Check directory permissions
   - Verify ClamAV is running: `sudo systemctl status clamav-daemon`
   - Check file size limits in Nginx and application config

### Getting Help

If you encounter issues not covered in this guide, consult:

1. Application documentation in the `/var/www/dev-forum/docs` directory
2. Log files for specific error messages
3. The project's issue tracker or community forums

## Maintenance

### Regular Updates

1. Update system packages regularly:

   ```bash
   sudo apt update && sudo apt upgrade -y
   ```

2. Update application code:

   ```bash
   cd /var/www/dev-forum
   sudo -u dev_forum git pull
   cd backend
   sudo -u dev_forum cargo build --release
   cd ../frontend
   sudo -u dev_forum npm install
   sudo -u dev_forum npm run build
   sudo systemctl restart dev-forum-backend
   ```

3. Renew SSL certificates:

   Certbot should renew certificates automatically. You can test the renewal process with:

   ```bash
   sudo certbot renew --dry-run
   ```

4. Database maintenance:

   ```bash
   sudo mysql -u root -p
   ```

   ```sql
   OPTIMIZE TABLE dev_forum.users, dev_forum.categories, dev_forum.threads, dev_forum.posts;
   ```

### Backup Verification

Periodically verify that backups are being created and can be restored:

```bash
# List recent backups
ls -la /var/backups/dev-forum/

# Test database backup integrity
gunzip -c /var/backups/dev-forum/db_XXXXXXXX.sql.gz | head -n 20
``` 