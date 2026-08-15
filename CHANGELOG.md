# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

https://github.com/gab1618/lapse/compare/v0.1.1..HEAD

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
