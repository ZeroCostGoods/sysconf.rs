# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [0.2.0] - 2017-07-22

### Added
- This CHANGELOG file
- A couple basic tests

### Changed
- `sysconf` function now returns c_long instead of i64. This potentially breaks backwards
  compatability but is necessary to work with 32-bit OS's
