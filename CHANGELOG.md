# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/vi17250/valinta/compare/v0.1.0...v0.2.0) - 2025-12-29

### Added

- [**breaking**] 🎸 Move cursor

### Fixed

- 🐛 the current item is preceded by an arrow

### Other

- Replace example image in README
- Update demo image in README.md
- 💡 number of rendered lines accept generic: &[T]
- release v0.1.0

## [0.1.0](https://github.com/vi17250/valinta/releases/tag/v0.1.0) - 2025-12-11

### Added

- 🎸 expose ValintaError

### Fixed

- 🐛 it works with all terminal width
- 🐛 scroll through the list if it's longer that the number of items to render
- 🐛 current line is highlighted
- 🐛 navigate through regardless of the terminal width
- 🐛 uses Clone instead of Copy
- 🐛 Option has 3 properties
- 🐛 clear terminal
- 🐛 restore multilines
- 🐛 disable line wrapping
- 🐛 expose only multi_select

### Other

- 🎡 Add release PR and push to crates.io
- ✏️ add emojies to README
- Update gif demo
- 🤖 disable doc test
- ✏️  document front page and ValintaError and update README
- ✏️ change header
- [**breaking**] 💡 follow `valinta-ts` structure 🪅
- ✏️ front page documentation
- 💡 `things` are now `items` and that is better
- 💡 only required modules are exposed and flattened
- Revert "fix: 🐛 clear terminal"
- ✏️ Header
- Include demo image in README
- ✏️ Update title
- header formatting in README.md
- 💡 module organisation
- 🎡 build + test + clippy on pr and push ([#6](https://github.com/vi17250/valinta/pull/6))
- 🤖 initial
