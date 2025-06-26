# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.11.1](https://github.com/librasn/compiler/compare/rasn-compiler-v0.11.0...rasn-compiler-v0.11.1) - 2025-06-26

### Other

- *(README)* specify REAL support

## [0.11.0](https://github.com/librasn/compiler/compare/rasn-compiler-v0.10.3...rasn-compiler-v0.11.0) - 2025-06-24

### Added

- *(lexer)* Support skipping over MACROs
- *(lexer)* Handle escaped `"` in character string value

### Fixed

- remove tags on anonymous types

## [0.10.3](https://github.com/librasn/compiler/compare/rasn-compiler-v0.10.2...rasn-compiler-v0.10.3) - 2025-06-14

### Fixed

- Backend selection was flipped

## [0.10.2](https://github.com/librasn/compiler/compare/rasn-compiler-v0.10.1...rasn-compiler-v0.10.2) - 2025-06-11

### Added

- feat(cli) Set non-zero exit code on error
- *(cli)* prefix output with colored severity
- *(cli)* report walk dir errors
- *(cli)* move -d and -m to required group

### Other

- Set MSRV

## [0.10.1](https://github.com/librasn/compiler/compare/rasn-compiler-v0.10.0...rasn-compiler-v0.10.1) - 2025-05-22

### Other

- Merge pull request #107 from librasn/refactor/dry
- VisibleString range endpoints are inclusive
- Fixes multiples character subset parsing, like: VisibleString(FROM ("a".."z" | "A".."Z" | "0".."9" | ".-"))

## [0.10.0](https://github.com/librasn/compiler/compare/rasn-compiler-v0.9.0...rasn-compiler-v0.10.0) - 2025-05-16

### Added

- add support for tagged sequence of items

### Other

- update dependencies

## [0.9.0](https://github.com/librasn/compiler/compare/rasn-compiler-v0.8.1...rasn-compiler-v0.9.0) - 2025-03-14

### Fixed

- *(linker)* build reference graph to stop at circular references in recursion detection

## [0.8.1](https://github.com/librasn/compiler/compare/rasn-compiler-v0.8.0...rasn-compiler-v0.8.1) - 2025-03-12

### Fixed

- *(linker)* prevent stack overflow from over-eager recursion marking

## [0.8.0](https://github.com/librasn/compiler/compare/rasn-compiler-v0.7.5...rasn-compiler-v0.8.0) - 2025-03-11

### Added

- *(rasn)* add custom imports and type annotations
- *(error)* implement Error for CompilerError

### Fixed

- add wasm compatibility

### Other

- Merge branch 'main' into feat/custom-imports-and-derives

## [0.7.5](https://github.com/librasn/compiler/compare/rasn-compiler-v0.7.4...rasn-compiler-v0.7.5) - 2025-03-03

### Fixed

- *(rasn)* correctly bind nested list item constraints

## [0.7.4](https://github.com/librasn/compiler/compare/rasn-compiler-v0.7.3...rasn-compiler-v0.7.4) - 2025-02-04

### Other

- enable graphic string

## [0.7.3](https://github.com/librasn/compiler/compare/rasn-compiler-v0.7.2...rasn-compiler-v0.7.3) - 2025-01-21

### Other

- Try a bit harder to find rustfmt
- Fix warning about named and elided lifetime use

## [0.7.2](https://github.com/librasn/compiler/compare/rasn-compiler-v0.7.1...rasn-compiler-v0.7.2) - 2025-01-07

### Fixed

- make null/any newtype fields public

## [0.7.1](https://github.com/librasn/compiler/compare/rasn-compiler-v0.7.0...rasn-compiler-v0.7.1) - 2024-12-15

### Fixed

- *(linker)* link defaults in nested choices
- error parsing single subtype constraint

## [0.7.0](https://github.com/librasn/compiler/compare/rasn-compiler-v0.6.1...rasn-compiler-v0.7.0) - 2024-12-02

### Added

- add support for named bits

## [0.6.1](https://github.com/librasn/compiler/compare/rasn-compiler-v0.6.0...rasn-compiler-v0.6.1) - 2024-11-13

### Fixed

- *(rasn)* handle deeply nested types

## [0.6.0](https://github.com/librasn/compiler/compare/rasn-compiler-v0.5.3...rasn-compiler-v0.6.0) - 2024-11-10

### Added

- *(error)* contextualize linker and grammar errors
- *(lexer)* return error snippet
- *(lexer)* collect errors in tree
- *(lexer)* return error pdu
- *(lexer)* custom input type

### Fixed

- return parsing errors for multi-module input
- *(cli)* add colored dependency
- *(lexer)* extract exact error snippet
- *(lexer)* tstring validation

### Other

- *(error)* return enum instead of boxed trait
- *(lexer)* use Input type

## [0.5.3](https://github.com/librasn/compiler/compare/rasn-compiler-v0.5.2...rasn-compiler-v0.5.3) - 2024-10-07

### Added

- *(parameterization)* support type-only params

### Fixed

- *(errors)* limit error output

## [0.5.2](https://github.com/librasn/compiler/compare/rasn-compiler-v0.5.1...rasn-compiler-v0.5.2) - 2024-10-01

### Other

- *(rasn)* use fixed variant for strictly constrained bit and octetstrings

## [0.5.1](https://github.com/librasn/compiler/compare/rasn-compiler-v0.5.0...rasn-compiler-v0.5.1) - 2024-09-26

### Fixed

- assure compatibility with rasn v0.19

## [0.5.0](https://github.com/librasn/compiler/compare/rasn-compiler-v0.4.0...rasn-compiler-v0.5.0) - 2024-09-19

### Added

- consider tagging and extensibility environments
- *(validator)* box direct recursive children

### Fixed

- *(wasm)* add field to wasm config constructor

## [0.4.0](https://github.com/librasn/compiler/compare/rasn-compiler-v0.3.2...rasn-compiler-v0.4.0) - 2024-09-16

### Other

- Handle case of same type in different variants
- Add optional From impl generation for CHOICE
- Fix a couple warnings & fmt

## [0.3.2](https://github.com/librasn/compiler/compare/rasn-compiler-v0.3.1...rasn-compiler-v0.3.2) - 2024-08-23

### Other
- fix enumerated comment descriptions
