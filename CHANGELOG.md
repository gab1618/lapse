# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

https://github.com/gab1618/lapse/compare/latest..HEAD

### Added

- List inline logs
- See logs details

## [0.3.0] - 2026-08-19

### Added

- gitignore to templates
- default env files
- Pretty colored log formatting
- Timestamp and duration informations to log
- Response showing pretty response log
- Default scheme configurable by env
- Env config file

### Fixed

- Template files being created on wrong paths
- Template directories not working
- Broken log entries would break the pager instead of just being ignored
- Trying to log request without logs would trigger an error instead of just showing no log
- Missing entries on .gitignore

## [0.2.2] - 2026-08-15

### Added

- Show current env on env ls command

### Changed

- Move tree traverse logic into core package

## [0.2.1] - 2026-08-15

### Changed

- Move space initialization logic to templates package
- Simplify template creation API

### Fixed

- Minor clippy warnings
- Remove empty module

## [0.2.0] - 2026-08-15

### Added

- Environment inheritance. Now envs inherit values from their parents

### Fixed

- Wrong author email on packages metadata
- Invalid env names being queried on switch command, due to flaws on the tree logic

## [0.1.1] - 2026-08-14

### Fixed

- CLI commands having all the same description

## [0.1.0] - 2026-08-14

### Added

- CLI package
  - Fuzzy selectors
  - Completion
  - Build
- Core package
  - Requests
  - Envs
  - Scripts
  - Hooks
- Template package
  - Basic presets (default and httpbin)
  - Generate from OpenAPI schemas
