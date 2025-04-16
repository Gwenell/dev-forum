# Developer Forum: API Reference

This document provides a comprehensive reference for the Developer Forum's REST API endpoints and WebSocket interfaces.

## Base URL

All REST API endpoints are prefixed with:

```
/api
```

## Authentication

Most API endpoints require authentication using a JWT token.

Include the token in the `Authorization` header:

```
Authorization: Bearer <your_jwt_token>
```

## Response Format

All REST API responses use a consistent JSON format:

```json
{
  "status": "success",
  "data": { ... }
}
```

For errors:

```json
{
  "status": "error",
  "message": "Error message"
}
```

## Endpoints

### Authentication

#### Register a new user

- **URL**: `/auth/register`
- **Method**: `POST`
- **Authentication**: None
- **Request Body**:
  ```json
  {
    "username": "username",
    "email": "user@example.com",
    "password": "password123"
  }
  ```
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "user_id": "uuid",
      "username": "username"
    }
  }
  ```

#### Login

- **URL**: `/auth/login`
- **Method**: `POST`
- **Authentication**: None
- **Request Body**:
  ```json
  {
    "username": "username",
    "password": "password123"
  }
  ```
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "token": "jwt_token",
      "user_id": "uuid",
      "username": "username",
      "is_admin": false
    }
  }
  ```

#### Validate Token

- **URL**: `/auth/validate`
- **Method**: `GET`
- **Authentication**: Required
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "user_id": "uuid",
      "username": "username",
      "is_admin": false
    }
  }
  ```

### Users

#### Get Current User

- **URL**: `/users/me`
- **Method**: `GET`
- **Authentication**: Required
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "username": "username",
      "email": "user@example.com",
      "display_name": "Display Name",
      "bio": "User bio",
      "avatar_url": "url_to_avatar",
      "theme_preference": "light",
      "is_admin": false,
      "created_at": "2023-01-01T00:00:00Z",
      "last_login": "2023-01-01T00:00:00Z"
    }
  }
  ```

#### Update User

- **URL**: `/users/me`
- **Method**: `PUT`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "display_name": "New Display Name",
    "bio": "Updated bio",
    "avatar_url": "new_avatar_url",
    "theme_preference": "dark"
  }
  ```
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "username": "username",
      "email": "user@example.com",
      "display_name": "New Display Name",
      "bio": "Updated bio",
      "avatar_url": "new_avatar_url",
      "theme_preference": "dark",
      "is_admin": false,
      "created_at": "2023-01-01T00:00:00Z",
      "last_login": "2023-01-01T00:00:00Z"
    }
  }
  ```

#### Change Password

- **URL**: `/users/password`
- **Method**: `POST`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "current_password": "current_password",
    "new_password": "new_password"
  }
  ```
- **Success Response**:
  ```json
  {
    "status": "success",
    "message": "Password updated successfully"
  }
  ```

#### Get User by ID

- **URL**: `/users/:user_id`
- **Method**: `GET`
- **Authentication**: None
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "username": "username",
      "display_name": "Display Name",
      "bio": "User bio",
      "avatar_url": "url_to_avatar",
      "created_at": "2023-01-01T00:00:00Z"
    }
  }
  ```

### Categories

#### Get All Categories

- **URL**: `/categories`
- **Method**: `GET`
- **Authentication**: None
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": [
      {
        "id": "uuid",
        "name": "Category Name",
        "slug": "category-slug",
        "description": "Category description",
        "icon": "icon_name",
        "display_order": 1,
        "subcategories": [
          {
            "id": "uuid",
            "category_id": "uuid",
            "name": "Subcategory Name",
            "slug": "subcategory-slug",
            "description": "Subcategory description",
            "icon": "icon_name",
            "display_order": 1
          }
        ]
      }
    ]
  }
  ```

#### Get Category by ID

- **URL**: `/categories/:category_id`
- **Method**: `GET`
- **Authentication**: None
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "name": "Category Name",
      "slug": "category-slug",
      "description": "Category description",
      "icon": "icon_name",
      "display_order": 1,
      "subcategories": [
        {
          "id": "uuid",
          "category_id": "uuid",
          "name": "Subcategory Name",
          "slug": "subcategory-slug",
          "description": "Subcategory description",
          "icon": "icon_name",
          "display_order": 1
        }
      ]
    }
  }
  ```

#### Create Category (Admin)

- **URL**: `/categories`
- **Method**: `POST`
- **Authentication**: Admin required
- **Request Body**:
  ```json
  {
    "name": "New Category",
    "description": "Category description",
    "icon": "icon_name",
    "display_order": 1
  }
  ```
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "name": "New Category",
      "slug": "new-category",
      "description": "Category description",
      "icon": "icon_name",
      "display_order": 1,
      "subcategories": []
    }
  }
  ```

#### Update Category (Admin)

- **URL**: `/categories/:category_id`
- **Method**: `PUT`
- **Authentication**: Admin required
- **Request Body**:
  ```json
  {
    "name": "Updated Category",
    "description": "Updated description",
    "icon": "new_icon",
    "display_order": 2
  }
  ```
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "name": "Updated Category",
      "slug": "updated-category",
      "description": "Updated description",
      "icon": "new_icon",
      "display_order": 2,
      "subcategories": [...]
    }
  }
  ```

### Subcategories

#### Create Subcategory (Admin)

- **URL**: `/categories/:category_id/subcategories`
- **Method**: `POST`
- **Authentication**: Admin required
- **Request Body**:
  ```json
  {
    "name": "New Subcategory",
    "description": "Subcategory description",
    "icon": "icon_name",
    "display_order": 1
  }
  ```
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "category_id": "uuid",
      "name": "New Subcategory",
      "slug": "new-subcategory",
      "description": "Subcategory description",
      "icon": "icon_name",
      "display_order": 1
    }
  }
  ```

#### Update Subcategory (Admin)

- **URL**: `/categories/:category_id/subcategories/:subcategory_id`
- **Method**: `PUT`
- **Authentication**: Admin required
- **Request Body**:
  ```json
  {
    "name": "Updated Subcategory",
    "description": "Updated description",
    "icon": "new_icon",
    "display_order": 2
  }
  ```
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "category_id": "uuid",
      "name": "Updated Subcategory",
      "slug": "updated-subcategory",
      "description": "Updated description",
      "icon": "new_icon",
      "display_order": 2
    }
  }
  ```

### Threads

#### Get Threads in Subcategory

- **URL**: `/subcategories/:subcategory_id/threads`
- **Method**: `GET`
- **Authentication**: None
- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `limit`: Items per page (default: 20)
  - `sort`: Sort field (default: "created_at")
  - `order`: Sort order (default: "desc")
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "threads": [
        {
          "id": "uuid",
          "subcategory_id": "uuid",
          "user_id": "uuid",
          "username": "username",
          "title": "Thread Title",
          "slug": "thread-slug",
          "is_pinned": false,
          "is_locked": false,
          "views": 10,
          "post_count": 5,
          "created_at": "2023-01-01T00:00:00Z",
          "updated_at": "2023-01-01T00:00:00Z",
          "last_post_at": "2023-01-01T00:00:00Z"
        }
      ],
      "total": 50,
      "page": 1,
      "limit": 20,
      "total_pages": 3
    }
  }
  ```

#### Get Thread by ID

- **URL**: `/threads/:thread_id`
- **Method**: `GET`
- **Authentication**: None
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "subcategory_id": "uuid",
      "subcategory_name": "Subcategory Name",
      "category_id": "uuid",
      "category_name": "Category Name",
      "user_id": "uuid",
      "username": "username",
      "user_avatar": "avatar_url",
      "title": "Thread Title",
      "slug": "thread-slug",
      "is_pinned": false,
      "is_locked": false,
      "views": 11,
      "created_at": "2023-01-01T00:00:00Z",
      "updated_at": "2023-01-01T00:00:00Z",
      "last_post_at": "2023-01-01T00:00:00Z"
    }
  }
  ```

#### Create Thread

- **URL**: `/subcategories/:subcategory_id/threads`
- **Method**: `POST`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "title": "New Thread Title",
    "content": "Initial post content",
    "code_content": "Optional code snippet",
    "code_language": "javascript"
  }
  ```
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "thread_id": "uuid",
      "thread_slug": "new-thread-title-a1b2c3",
      "post_id": "uuid"
    }
  }
  ```

#### Update Thread (Admin/Owner)

- **URL**: `/threads/:thread_id`
- **Method**: `PUT`
- **Authentication**: Required (Owner or Admin)
- **Request Body**:
  ```json
  {
    "title": "Updated Thread Title",
    "is_pinned": true,
    "is_locked": false
  }
  ```
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "title": "Updated Thread Title",
      "slug": "updated-thread-title",
      "is_pinned": true,
      "is_locked": false
    }
  }
  ```

### Posts

#### Get Posts in Thread

- **URL**: `/threads/:thread_id/posts`
- **Method**: `GET`
- **Authentication**: None
- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `limit`: Items per page (default: 20)
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "posts": [
        {
          "id": "uuid",
          "thread_id": "uuid",
          "user_id": "uuid",
          "username": "username",
          "display_name": "Display Name",
          "avatar_url": "avatar_url",
          "content": "Post content",
          "code_content": "Code snippet",
          "code_language": "javascript",
          "is_edited": false,
          "created_at": "2023-01-01T00:00:00Z",
          "updated_at": "2023-01-01T00:00:00Z"
        }
      ],
      "total": 25,
      "page": 1,
      "limit": 20,
      "total_pages": 2
    }
  }
  ```

#### Create Post (Reply)

- **URL**: `/threads/:thread_id/posts`
- **Method**: `POST`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "content": "Reply content",
    "code_content": "Optional code snippet",
    "code_language": "python"
  }
  ```
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "thread_id": "uuid",
      "user_id": "uuid",
      "content": "Reply content",
      "code_content": "Optional code snippet",
      "code_language": "python",
      "is_edited": false,
      "created_at": "2023-01-01T00:00:00Z",
      "updated_at": "2023-01-01T00:00:00Z"
    }
  }
  ```

#### Update Post

- **URL**: `/posts/:post_id`
- **Method**: `PUT`
- **Authentication**: Required (Owner or Admin)
- **Request Body**:
  ```json
  {
    "content": "Updated content",
    "code_content": "Updated code snippet",
    "code_language": "rust"
  }
  ```
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "thread_id": "uuid",
      "user_id": "uuid",
      "content": "Updated content",
      "code_content": "Updated code snippet",
      "code_language": "rust",
      "is_edited": true,
      "created_at": "2023-01-01T00:00:00Z",
      "updated_at": "2023-01-01T00:00:00Z"
    }
  }
  ```

### Files

#### Upload File

- **URL**: `/files/upload`
- **Method**: `POST`
- **Authentication**: Required
- **Content-Type**: `multipart/form-data`
- **Form Fields**:
  - `file`: File data
  - `description`: File description
  - `platform`: Platform (windows, macos, linux, etc.)
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "file_name": "file.zip",
      "file_path": "/uploads/user123/file.zip",
      "file_size": 1024,
      "mime_type": "application/zip",
      "description": "File description",
      "platform": "windows",
      "download_count": 0,
      "is_malware_scanned": true,
      "is_safe": true,
      "created_at": "2023-01-01T00:00:00Z",
      "updated_at": "2023-01-01T00:00:00Z"
    }
  }
  ```

#### Get Files

- **URL**: `/files`
- **Method**: `GET`
- **Authentication**: None
- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `limit`: Items per page (default: 20)
  - `platform`: Filter by platform
  - `sort`: Sort field (default: "created_at")
  - `order`: Sort order (default: "desc")
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "files": [
        {
          "id": "uuid",
          "user_id": "uuid",
          "username": "username",
          "file_name": "file.zip",
          "file_size": 1024,
          "mime_type": "application/zip",
          "description": "File description",
          "platform": "windows",
          "download_count": 5,
          "created_at": "2023-01-01T00:00:00Z"
        }
      ],
      "total": 40,
      "page": 1,
      "limit": 20,
      "total_pages": 2
    }
  }
  ```

#### Download File

- **URL**: `/files/:file_id/download`
- **Method**: `GET`
- **Authentication**: None
- **Response**: File download

### Streams

#### Get Active Streams

- **URL**: `/streams/active`
- **Method**: `GET`
- **Authentication**: None
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "streams": [
        {
          "id": "uuid",
          "user_id": "uuid",
          "username": "username",
          "display_name": "Display Name",
          "avatar_url": "avatar_url",
          "title": "Stream Title",
          "description": "Stream description",
          "is_active": true,
          "resolution": "1920x1080",
          "refresh_rate": 60,
          "viewer_count": 10,
          "started_at": "2023-01-01T00:00:00Z"
        }
      ]
    }
  }
  ```

#### Start Stream

- **URL**: `/streams/start`
- **Method**: `POST`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "title": "Stream Title",
    "description": "Stream description",
    "resolution": "1920x1080",
    "refresh_rate": 60
  }
  ```
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "stream_key": "unique_stream_key",
      "title": "Stream Title",
      "description": "Stream description",
      "resolution": "1920x1080",
      "refresh_rate": 60
    }
  }
  ```

#### End Stream

- **URL**: `/streams/:stream_id/end`
- **Method**: `POST`
- **Authentication**: Required (Owner)
- **Success Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "uuid",
      "ended_at": "2023-01-01T01:00:00Z",
      "duration_seconds": 3600,
      "max_viewers": 15
    }
  }
  ```

## WebSocket API

### Connection

Connect to the WebSocket server:

```
ws://hostname/ws
```

Authentication is required via query parameter:

```
ws://hostname/ws?token=your_jwt_token
```

### Message Format

All WebSocket messages use a JSON format:

```json
{
  "type": "message_type",
  "data": { ... }
}
```

### Chat Messages

#### Join Room

```json
{
  "type": "join_room",
  "data": {
    "room_id": "room_id"
  }
}
```

#### Leave Room

```json
{
  "type": "leave_room",
  "data": {
    "room_id": "room_id"
  }
}
```

#### Send Message

```json
{
  "type": "chat_message",
  "data": {
    "room_id": "room_id",
    "message": "Hello world!",
    "is_code": false,
    "code_language": null
  }
}
```

#### Send Code Message

```json
{
  "type": "chat_message",
  "data": {
    "room_id": "room_id",
    "message": "function hello() { return 'world'; }",
    "is_code": true,
    "code_language": "javascript"
  }
}
```

#### Receive Message

```json
{
  "type": "chat_message",
  "data": {
    "id": "uuid",
    "room_id": "room_id",
    "user_id": "uuid",
    "username": "username",
    "display_name": "Display Name",
    "avatar_url": "avatar_url",
    "message": "Hello world!",
    "is_code": false,
    "code_language": null,
    "created_at": "2023-01-01T00:00:00Z"
  }
}
```

### Stream Signaling

#### Signal Offer

```json
{
  "type": "signal_offer",
  "data": {
    "stream_id": "uuid",
    "offer": "webrtc_offer_data"
  }
}
```

#### Signal Answer

```json
{
  "type": "signal_answer",
  "data": {
    "stream_id": "uuid",
    "answer": "webrtc_answer_data"
  }
}
```

#### Signal ICE Candidate

```json
{
  "type": "signal_ice",
  "data": {
    "stream_id": "uuid",
    "candidate": "ice_candidate_data"
  }
}
```

#### Viewer Count Update

```json
{
  "type": "viewer_count",
  "data": {
    "stream_id": "uuid",
    "count": 10
  }
}
``` 