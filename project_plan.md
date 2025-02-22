# IPTV Streaming Application Project Plan

## Project Overview
A cross-platform desktop application for streaming IPTV content from multiple service providers. The application will manage IPTV playlists, channel information, and provide a seamless streaming experience.

## Technical Stack
- **Backend**
  - Tauri (Desktop application framework)
  - Rust (Backend logic)
  - SQLite (Local database)
  - IPTV stream handling libraries
  - FFmpeg (for video playback)

- **Frontend**
  - Svelte 5 (UI framework)
  - Shadcn-svelte (UI component library)
  - Video.js (Video player)
  - TypeScript
  - Vite (Build tool)

## Feature Requirements

### 1. Playlist Management
- Add/Edit/Delete IPTV service provider accounts
- Store multiple playlists with unique names
- Fields for each playlist:
  - Provider name
  - Server URL
  - Username
  - Password
  - Last updated timestamp

### 2. Channel Management
- Automatic playlist parsing and channel extraction
- Channel categorization (News, Sports, Entertainment, etc.)
- Channel metadata storage (name, ID, category, logo, etc.)
- Regular playlist synchronization
- Channel search and filtering

### 3. Video Playback
- Live stream playback
- Channel switching
- Video quality settings
- Playback controls
- EPG (Electronic Program Guide) integration if available
- Stream health monitoring

### 4. User Interface
- Clean, modern design using Shadcn-svelte
- Responsive layouts
- Dark/Light theme support
- Category-based channel browsing
- Favorites management
- Search functionality
- Settings panel

## Database Schema

### Playlists Table
```sql
CREATE TABLE playlists (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    server_url TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    last_updated TIMESTAMP,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### Channels Table
```sql
CREATE TABLE channels (
    id INTEGER PRIMARY KEY,
    playlist_id INTEGER,
    channel_id TEXT NOT NULL,
    name TEXT NOT NULL,
    category TEXT,
    logo_url TEXT,
    stream_url TEXT NOT NULL,
    is_favorite BOOLEAN DEFAULT false,
    last_checked TIMESTAMP,
    is_working BOOLEAN DEFAULT true,
    FOREIGN KEY (playlist_id) REFERENCES playlists(id)
);
```

### Categories Table
```sql
CREATE TABLE categories (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id INTEGER,
    FOREIGN KEY (parent_id) REFERENCES categories(id)
);
```

### Channel_Categories Table
```sql
CREATE TABLE channel_categories (
    channel_id INTEGER,
    category_id INTEGER,
    PRIMARY KEY (channel_id, category_id),
    FOREIGN KEY (channel_id) REFERENCES channels(id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);
```

## Architecture Components

### Backend (Rust/Tauri)
1. **Database Manager**
   - SQLite connection management
   - Query execution
   - Migration handling

2. **Playlist Manager**
   - M3U8 playlist parsing
   - Channel information extraction
   - Playlist synchronization

3. **Stream Handler**
   - Stream URL validation
   - Stream health checking
   - Video stream management

4. **API Layer**
   - IPC (Inter-Process Communication) endpoints
   - Frontend-Backend communication
   - Event handling

### Frontend (Svelte 5)
1. **State Management**
   - Playlist state
   - Channel state
   - UI state
   - Settings state

2. **UI Components**
   - Navigation sidebar
   - Channel grid/list
   - Video player
   - Settings panel
   - Playlist manager
   - Category browser

3. **Services**
   - API client
   - Local storage management
   - Theme management
   - Channel category management

## Implementation Phases

### Phase 1: Project Setup (1 week)
- Set up Tauri with Rust backend
- Configure Svelte 5 frontend
- Integrate Shadcn-svelte
- Set up development environment
- Initialize SQLite database

### Phase 2: Core Features (2 weeks)
- Implement database schema
- Create playlist management system
- Develop channel parsing and storage
- Build basic UI components

### Phase 3: Video Integration (2 weeks)
- Implement video player
- Add stream handling
- Create channel switching logic
- Add basic playback controls

### Phase 4: UI Enhancement (1 week)
- Implement complete UI using Shadcn-svelte
- Add theme support
- Create responsive layouts
- Implement search and filtering

### Phase 5: Testing & Polish (1 week)
- Comprehensive testing
- Bug fixes
- Performance optimization
- User feedback integration

## Dependencies and Prerequisites
- Rust and Cargo
- Node.js and npm/yarn
- FFmpeg
- SQLite
- Development tools (VS Code recommended)
- Git for version control

## Project Structure
```
iptv-app/
├── src-tauri/               # Rust backend code
│   ├── src/
│   │   ├── main.rs         # Main Tauri application
│   │   ├── db/             # Database management
│   │   ├── playlist/       # Playlist handling
│   │   ├── stream/         # Stream management
│   │   └── api/            # Backend API endpoints
│   └── Cargo.toml
├── src/                     # Svelte frontend code
│   ├── lib/                # Shared components
│   ├── routes/             # Page components
│   ├── services/           # Frontend services
│   └── app.svelte          # Main application
├── static/                  # Static assets
├── package.json
└── README.md
```

## Security Considerations
- Secure storage of playlist credentials
- Stream URL validation
- Input sanitization
- Error handling
- Update mechanism
- Credential encryption

## Testing Strategy
- Unit tests for Rust backend
- Integration tests for API endpoints
- Component tests for Svelte frontend
- End-to-end testing
- Performance testing
- Security testing

## Next Steps
1. Set up development environment
2. Create project structure
3. Initialize database
4. Begin implementation of Phase 1

## Success Metrics
- Successful playlist parsing and management
- Smooth video playback
- Responsive UI
- Efficient channel switching
- Stable performance
- User satisfaction

This plan will be refined and updated as development progresses.