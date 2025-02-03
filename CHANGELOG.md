## [2025-02-03] [PR#13](https://github.com/KennyDizi/DCA/pull/13)

### Added
- Added `extra_fields` to handle unexpected JSON fields in API responses.
- Introduced `ResponseParse` error type with raw response details.
- Added cost logging for `deepseek` and `anthropic` API calls.

### Changed
- Enhanced error handling for API response parsing and HTTP status codes.
- Improved error response formats and messages.

## [2025-02-03] [PR#11](https://github.com/KennyDizi/DCA/pull/11)

### Changed
- Modified chat handler to return OpenAI-compatible JSON response format.
- Reordered response content blocks to prioritize Anthropic output before reasoning content.

### Added
- Debug logging for assistant response content and request timestamps.
- Request logging in health check handler.

## [2025-02-02] [PR#9](https://github.com/KennyDizi/DCA/pull/9)

### Changed
- Updated server configuration to use port `11434` and host `0.0.0.0`.
- Renamed pricing models from `claude_3_sonnet` and `claude_3_haiku` to `claude_3_5_sonnet` and `claude_3_5_haiku`.
- Restructured `DeepSeekPricing` to include `DeepSeekModelPricing` configuration.

### Added
- Implemented health check API endpoint `/health`.

## [2025-01-31] [PR#7](https://github.com/KennyDizi/DCA/pull/7)

### Changed
- Renamed package from `deepclaudeadvanced` to `deepclaude` in `Cargo.toml`.
- Updated `Dockerfile` to use consistent casing for `AS` keyword.

### Fixed
- Removed trailing spaces and aligned error handling blocks in `src/clients/deepseek.rs`.