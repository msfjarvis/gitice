# gitice [![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/) [![Built with Garnix](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgarnix.io%2Fapi%2Fbadges%2Fmsfjarvis%2Fgitice%3Fbranch%3Dmain)](https://garnix.io)

Rust tool to create a 'lockfile' of the git repositories present under a directory, and allow re-cloning them when given the previously generated lockfile.

The purpose of this is to be able to quickly restore frequently used git repositories in event of an OS reinstall or things like that. I personally store my work on my OS drive since it is an SSD so in the event of me switching between operating systems I will have to manually clone my projects again which is tedious and annoying, and something I aim to make easier with this.
