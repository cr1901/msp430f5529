# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.0] - 2022-01-25

The crate was regenerated several times with various commits of msp430_svd
and svdrust, up to and including svd2rust v0.20.0; these commits (except for
the svd2rust v0.20.0 regeneration) were squashed. Breaking changes are due to
a newer svd2rust API and incompatible `CriticalSection<'a>` from the bare-metal,
msp430, and msp430-rt v0.3.0 dependencies.

No post-processing has been done on the output SVD, except to import the
`generic` module from a separate file. This will be done in future minor
(breaking) releases.

## v0.1.0 - 2021-03-23

Initial release. No [post-processing](https://github.com/pftbest/msp430_svd#patching)
has been done on the output SVD, except to import the `generic` module from
a separate file.

[Unreleased]: https://github.com/cr1901/msp430f5529/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/cr1901/msp430f5529/compare/v0.1.0...v0.2.0
