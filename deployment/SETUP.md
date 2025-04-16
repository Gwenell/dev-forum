# Developer Forum Setup

This document outlines the steps taken to set up the Developer Forum application.

## System Dependencies

The following dependencies have been installed on the system:

### Rust and Cargo
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

### Essential Build Tools
```bash
sudo apt-get install -y curl build-essential pkg-config libssl-dev
```

### Node.js and npm
```bash
sudo apt-get install -y nodejs npm
```

### Database Client
```bash
sudo apt-get install -y default-mysql-client default-libmysqlclient-dev
```

### ClamAV for Virus Scanning
```bash
sudo apt-get install -y clamav clamav-daemon
```

### Rust Libraries
The project uses the following Rust dependencies:
- Actix Web framework and related libraries
- Sea-ORM for database operations
- Authentication libraries (JWT, Argon2)
- ClamAV integration
- WebSocket support

### Frontend Libraries
The SvelteKit frontend uses:
- Socket.io for real-time communication
- Marked and highlight.js for code display
- PeerJS for WebRTC screen sharing
- TailwindCSS with typography and forms plugins

## Database Setup

Create the MariaDB database:

```bash
mysql -u root -p
```

Then in the MySQL console:
```sql
CREATE DATABASE dev_forum;
EXIT;
```

## Environment Configuration

Environment variables are stored in `.env` files in both the backend and frontend directories.

## Building and Running

### Backend
```bash
cd backend
cargo build --release
cargo run --release
```

### Frontend
```bash
cd frontend
npm run build
npm run preview
```

## Production Deployment

For a production deployment, consider:
- Setting up a reverse proxy with Nginx
- Configuring SSL certificates
- Setting up systemd services for automatic startup
- Using PM2 for Node.js process management 