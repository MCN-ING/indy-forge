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
- Improved DID handling and cryptographic operations
    - Implementation serves as cross-validation between indy-data-types and aries-askar libraries
    - Validates DID:SOV and DID:INDY spec compliance across both implementations
    - Confirms identical DID generation from same seed material
    - Verifies cryptographic compatibility between libraries
    - Ensures consistent verkey generation across implementations

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
- Cryptographic implementation improvements
    - Maintained reference implementation using indy-data-types
    - Validated against aries-askar implementation
    - Ensured cross-library compatibility
    - Verified consistent cryptographic operations

### Fixed

- Fixed infinite connection retry on failed URLs
- Fixed borrowing issues with error handling
- Fixed connection state not resetting properly
- Fixed error messages not displaying correctly
- Fixed genesis file content not updating on source change
- Fixed UI state preservation during connection attempts
- Fixed connection timeout handling
- Fixed error state management during retries
- Fixed signing operations
    - Verified consistent signing behavior across implementations
    - Validated signature compatibility
    - Ensured deterministic DID generation
    - Confirmed cross-library cryptographic material compatibility

### Enhanced Logging

- Added debug logging for genesis file operations
- Added connection attempt logging
- Added error state logging
- Added HTTP request/response logging
- Added timeout and connection state logging
- Added cryptographic operation logging
    - Key derivation process logging
    - Signing operation details
    - DID creation steps
    - Transaction signing verification

### Security

- Improved key handling
    - Validated cryptographic implementations against specifications
    - Verified consistent key generation across libraries
    - Confirmed deterministic key derivation
    - Cross-validated cryptographic operations

### Code Quality

- Improved architecture
    - Maintained reference implementation for cross-validation
    - Demonstrated spec compliance across libraries
    - Provided compatibility verification
    - Enhanced testing coverage through cross-implementation validation

### Verification

- Added cross-implementation validation
    - Verified DID:SOV spec compliance between indy-data-types and aries-askar
    - Confirmed DID:INDY spec compliance across implementations
    - Validated consistent verkey generation
    - Demonstrated cryptographic compatibility
    - Ensured deterministic output from identical inputs