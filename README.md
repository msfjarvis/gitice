# gitice [![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

Rust tool to create a 'lockfile' of the git repositories present under a directory, and allow re-cloning them when given the previously generated lockfile.

The purpose of this is to be able to quickly restore frequently used git repositories in event of an OS reinstall or things like that. I personally store my work on my OS drive since it is an SSD so in the event of me switching between operating systems I will have to manually clone my projects again which is tedious and annoying, and something I aim to make easier with this.
