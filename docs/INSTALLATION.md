# Developer Forum: Installation Guide

This guide provides step-by-step instructions for setting up the Developer Forum application on a Debian-based system.

## Prerequisites

- Debian-based operating system (Debian, Ubuntu, etc.)
- Root access or sudo privileges
- MariaDB server (already installed)
- Internet connection for downloading dependencies

## System Dependencies Installation

### 1. Update System Packages

```bash
sudo apt update
sudo apt upgrade -y
```

### 2. Install Rust and Cargo

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

### 3. Install Essential Build Tools

```bash
sudo apt install -y curl build-essential pkg-config libssl-dev
```

### 4. Install Node.js and npm

```bash
sudo apt install -y nodejs npm
```

For a more recent version of Node.js:

```bash
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs
```

### 5. Install Database Client Libraries

```bash
sudo apt install -y default-mysql-client default-libmysqlclient-dev
```

### 6. Install ClamAV for Virus Scanning

```bash
sudo apt install -y clamav clamav-daemon
sudo systemctl restart clamav-daemon
```

## Database Setup

### 1. Create the Database

```bash
cd /var/www/dev-forum/deployment
chmod +x setup_database.sh
./setup_database.sh
```

Alternatively, create the database manually:

```bash
mysql -u root -p212002
```

In the MySQL console:

```sql
CREATE DATABASE dev_forum CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
EXIT;
```

## Application Setup

### 1. Clone the Repository (if not already done)

```bash
git clone [repository_url] /var/www/dev-forum
cd /var/www/dev-forum
```

### 2. Backend Configuration

Create a `.env` file in the backend directory:

```bash
cd /var/www/dev-forum/backend
```

Add the following content to `.env`:

```
DATABASE_URL=mysql://root:212002@localhost/dev_forum
JWT_SECRET=your_secret_key_here
PORT=8000
FRONTEND_URL=http://localhost:3000
MAX_UPLOAD_SIZE=20971520
```

Replace `your_secret_key_here` with a strong random secret.

### 3. Frontend Configuration

Create a `.env` file in the frontend directory:

```bash
cd /var/www/dev-forum/frontend
```

Add the following content to `.env`:

```
VITE_API_URL=http://localhost:8000/api
VITE_WS_URL=ws://localhost:8000/ws
```

### 4. Build the Backend

```bash
cd /var/www/dev-forum/backend
cargo build --release
```

### 5. Build the Frontend

```bash
cd /var/www/dev-forum/frontend
npm install
npm run build
```

## Running the Application

### Development Mode

#### Backend:

```bash
cd /var/www/dev-forum/backend
cargo run
```

#### Frontend:

```bash
cd /var/www/dev-forum/frontend
npm run dev
```

### Production Mode

#### Backend:

```bash
cd /var/www/dev-forum/backend
cargo run --release
```

#### Frontend:

```bash
cd /var/www/dev-forum/frontend
npm run build
npm run preview
```

## Setting Up as a Service

### Backend Service

Create a systemd service file:

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
User=www-data
WorkingDirectory=/var/www/dev-forum/backend
ExecStart=/var/www/dev-forum/backend/target/release/dev-forum-backend
Restart=on-failure
Environment=DATABASE_URL=mysql://root:212002@localhost/dev_forum
Environment=JWT_SECRET=your_secret_key_here
Environment=PORT=8000
Environment=FRONTEND_URL=http://localhost:3000
Environment=MAX_UPLOAD_SIZE=20971520

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl enable dev-forum-backend
sudo systemctl start dev-forum-backend
```

### Frontend Service

For the frontend, you can use a web server like Nginx or use PM2:

#### Using PM2:

Install PM2:

```bash
sudo npm install -g pm2
```

Create a PM2 configuration file:

```bash
cd /var/www/dev-forum/frontend
```

Start the frontend with PM2:

```bash
pm2 start npm --name "dev-forum-frontend" -- run preview
pm2 save
pm2 startup
```

## Troubleshooting

- **Database Connection Issues**: Ensure MariaDB is running and the credentials are correct
- **Permission Issues**: Make sure the application has the necessary permissions to access files
- **ClamAV Issues**: Verify that the ClamAV daemon is running with `systemctl status clamav-daemon`

## Next Steps

After installation, follow these steps:

1. Access the application at `http://localhost:3000`
2. Create an admin account using the registration page
3. Configure the forum categories and subcategories

For production deployment, consider setting up:
- HTTPS with Let's Encrypt certificates
- A reverse proxy with Nginx
- Additional security measures like fail2ban 