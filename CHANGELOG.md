# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2024-11-11

### Added

- Genesis file viewing capability
    - Support for both local files and URLs
    - Content preview with copy functionality
    - Scrollable view for genesis content
    - URL-based genesis file loading
- Connection status monitoring and feedback
    - Visual connection status indicators
    - Periodic connection health checks (every 30 seconds)
    - Detailed error messages and logging
    - Real-time connection status updates
- Timeout mechanisms
    - HTTP client timeout (10 seconds)
    - Content reading timeout (5 seconds)
    - Overall operation timeout (15 seconds)
    - UI-level connection timeout (20 seconds)
    - Visual feedback for connection duration

### Changed

- Improved error handling
    - Better error messages for HTTP failures (404, etc.)
    - Clear error states with retry options
    - Separated error display from error handling logic
    - Added HTTP status code specific error messages
- Enhanced connection management
    - State tracking for genesis file changes
    - Proper reset of connection state on errors
    - Prevention of connection retry spam
    - Connection state cleanup on source changes
- UI Improvements
    - Added genesis content viewer toggle
    - Better error message formatting
    - Added retry button for failed connections
    - Added spinner during connection attempts
    - Separated wallet and genesis file checks
    - Progress indicators for long operations

### Fixed

- Fixed infinite connection retry on failed URLs
- Fixed borrowing issues with error handling
- Fixed connection state not resetting properly
- Fixed error messages not displaying correctly
- Fixed genesis file content not updating on source change
- Fixed UI state preservation during connection attempts
- Fixed connection timeout handling
- Fixed error state management during retries

### Enhanced Logging

- Added debug logging for genesis file operations
- Added connection attempt logging
- Added error state logging
- Added HTTP request/response logging
- Added timeout and connection state logging

