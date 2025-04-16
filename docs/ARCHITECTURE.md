# Developer Forum: Architecture Overview

This document provides a comprehensive overview of the Developer Forum application's architecture, detailing the technologies used, system design, and component interactions.

## System Architecture

The Developer Forum is built as a full-stack web application with the following architecture:

```
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│   Frontend    │◄────►│    Backend    │◄────►│   Database    │
│   (SvelteKit) │      │  (Rust/Actix) │      │   (MariaDB)   │
└───────────────┘      └───────────────┘      └───────────────┘
        ▲                      ▲                      ▲
        │                      │                      │
        ▼                      ▼                      ▼
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│  WebRTC P2P   │      │   WebSocket   │      │    Storage    │
│ Screen Sharing│      │ (Chat/Stream) │      │(Files/Uploads)│
└───────────────┘      └───────────────┘      └───────────────┘
                                                     ▲
                                                     │
                                                     ▼
                                              ┌───────────────┐
                                              │  File Scanner │
                                              │   (ClamAV)    │
                                              └───────────────┘
```

## Technology Stack

### Backend

- **Programming Language**: Rust
- **Web Framework**: Actix Web
- **ORM**: Sea-ORM
- **Authentication**: JSON Web Tokens (JWT)
- **Password Hashing**: Argon2
- **WebSockets**: Actix WebSocket
- **File Scanning**: ClamAV integration
- **Database Access**: Sea-Query

### Frontend

- **Framework**: SvelteKit
- **UI Styling**: TailwindCSS
- **State Management**: Svelte stores
- **API Communication**: Fetch API
- **WebSocket Client**: Socket.io client
- **Code Highlighting**: highlight.js
- **WebRTC**: PeerJS

### Database

- **DBMS**: MariaDB
- **Schema Management**: Sea-ORM migrations
- **Data Structure**: Relational schema

### Infrastructure

- **Operating System**: Debian
- **Deployment**: Systemd services
- **Process Management**: PM2 (for Node.js)

## Component Breakdown

### Backend Components

#### 1. API Layer

The API layer provides HTTP endpoints for the frontend to interact with the application data and services.

- **Routes**: Organized by resource type (users, categories, threads, posts, files, chat, streams)
- **Controllers**: Handle request validation, business logic, and response formatting
- **Middleware**: Handle authentication, authorization, and request processing

#### 2. Model Layer

The model layer defines the database schema and provides methods for data access and manipulation.

- **Entities**: Represent database tables with typed fields and relationships
- **Migrations**: Handle database schema evolution
- **Active Records**: Provide an ORM interface to the database

#### 3. Service Layer

The service layer contains business logic and cross-cutting concerns.

- **Authentication Services**: Handle user authentication and session management
- **File Services**: Handle file uploads, storage, and scanning
- **WebSocket Services**: Manage real-time communication

#### 4. Utility Modules

Utility modules provide helper functions and shared functionality.

- **Error Handling**: Standardized error types and responses
- **Validation**: Input validation and sanitization
- **Configuration**: Application settings and environment variables

### Frontend Components

#### 1. Routing and Pages

- **Routes**: Defines the application URL structure
- **Pages**: Svelte components that render for specific routes
- **Layouts**: Shared UI structure across multiple pages

#### 2. Components

- **UI Components**: Reusable interface elements (buttons, forms, cards, etc.)
- **Feature Components**: Specialized components for specific features (code editor, file uploader, etc.)
- **Layout Components**: Structure the overall page layout

#### 3. Stores

- **Auth Store**: Manages user authentication state
- **Theme Store**: Manages theme preferences
- **Feature-specific Stores**: Manage state for specific features

#### 4. Services

- **API Service**: Handles communication with the backend API
- **WebSocket Service**: Manages real-time communication
- **WebRTC Service**: Manages peer-to-peer screen sharing

## Database Schema

The database uses a relational schema with the following main tables:

1. **users**: User account information
2. **categories**: Top-level forum categories
3. **subcategories**: Forum subcategories within categories
4. **threads**: Discussion threads within subcategories
5. **posts**: Individual messages within threads
6. **files**: Uploaded files metadata
7. **chat_messages**: Messages sent in real-time chat
8. **streams**: Live streaming session metadata

## Authentication Flow

1. User submits login credentials
2. Backend validates credentials against stored hash
3. If valid, backend generates JWT token
4. Token is returned to frontend and stored
5. Token is included in subsequent API requests
6. Backend validates token for authenticated routes

## Real-time Communication

### Chat System

1. Frontend establishes WebSocket connection
2. Messages are sent and received via WebSocket
3. Messages are persisted to the database
4. Historical messages are loaded via REST API

### Screen Sharing

1. Peer-to-peer connection established via WebRTC
2. Signaling server facilitates initial connection
3. Video stream is transmitted directly between peers
4. Metadata about streams is stored in the database

## File Handling

1. User uploads file via frontend
2. Backend receives file and stores temporarily
3. ClamAV scans file for malware
4. If clean, file is moved to permanent storage
5. Metadata is stored in the database
6. File is available for download via API

## Security Measures

1. **Authentication**: JWT with proper expiration and validation
2. **Password Security**: Argon2 hashing for passwords
3. **Input Validation**: All user input is validated and sanitized
4. **File Security**: Malware scanning for uploaded files
5. **CORS**: Properly configured for production environments
6. **Rate Limiting**: Prevents abuse of API endpoints
7. **SQL Injection Protection**: Parameterized queries via ORM

## Scalability Considerations

The architecture supports horizontal scaling through:

1. **Stateless Backend**: Allows multiple instances
2. **Database Scaling**: Supports replication and sharding
3. **File Storage**: Can be moved to distributed storage
4. **WebSocket Clustering**: Supports multiple nodes with shared state

## Development Workflow

1. **Local Development**: Run backend and frontend locally
2. **Testing**: Unit and integration tests
3. **CI/CD**: Automated builds and deployments
4. **Deployment**: Systemd services for production 