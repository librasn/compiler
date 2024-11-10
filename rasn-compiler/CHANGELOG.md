# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
