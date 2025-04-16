# Developer Forum Documentation

Welcome to the Developer Forum documentation. This collection of documents provides comprehensive information on installing, configuring, using, and maintaining the Developer Forum application.

## Documentation Index

### For Users

- [User Guide](USER_GUIDE.md) - Complete guide to using the Developer Forum application
- [FAQ](FAQ.md) - Frequently asked questions about using the forum

### For Administrators

- [Installation Guide](INSTALLATION.md) - Getting started with a basic installation
- [Production Deployment](PRODUCTION_DEPLOYMENT.md) - Comprehensive guide for deploying in production
- [Security Best Practices](SECURITY.md) - Security considerations and hardening guidelines
- [Backup and Recovery](BACKUP_RECOVERY.md) - Managing backups and disaster recovery

### For Developers

- [Architecture Overview](ARCHITECTURE.md) - System design and component interactions
- [API Reference](API_REFERENCE.md) - Complete API documentation
- [Development Setup](DEVELOPMENT_SETUP.md) - Setting up a development environment
- [Contributing Guidelines](CONTRIBUTING.md) - Guidelines for contributing to the project

## Project Structure

The Developer Forum project is organized as follows:

```
dev-forum/
├── backend/             # Rust backend implementation
│   ├── src/             # Backend source code
│   └── Cargo.toml       # Rust dependencies
├── frontend/            # SvelteKit frontend implementation
│   ├── src/             # Frontend source code
│   └── package.json     # Node.js dependencies
├── deployment/          # Deployment scripts and instructions
│   ├── setup_database.sh  # Database setup script
│   └── SETUP.md         # Setup instructions
└── docs/                # Documentation (you are here)
```

## Features

The Developer Forum includes the following key features:

1. **Forum Structure** - Categories, subcategories, and threads
2. **User Management** - Registration, authentication, and profiles
3. **Real-time Chat** - Live chat functionality between users
4. **Code Sharing** - Syntax highlighting for code snippets
5. **Live Screen Sharing** - WebRTC-based peer-to-peer screen sharing
6. **File Sharing** - Secure file uploads with malware scanning
7. **Theming** - Support for light and dark themes

## Technology Stack

The Developer Forum is built using the following technologies:

- **Backend**: Rust with Actix Web framework
- **Frontend**: SvelteKit
- **Database**: MariaDB
- **File Scanning**: ClamAV
- **Authentication**: JWT (JSON Web Tokens)

## Support and Community

For additional support or to connect with the community:

- [GitHub Repository](https://github.com/yourusername/dev-forum)
- [Community Forum](https://example.com/forum)
- [Discord Server](https://discord.gg/example)

## License

The Developer Forum is released under the [MIT License](../LICENSE).

## Contributing

We welcome contributions to both the codebase and documentation. Please see our [Contributing Guidelines](CONTRIBUTING.md) for details on how to get involved. 