# kernel-examples
Rust Linux kernel module examples

This repository has small sample Rust kernel driver examples.
Originally done for an internal class.

# Prerequisite
Building these examples assume you already have all the necessary
packages to build a Rust kernel module and headers for your distribution.

# Building module
Each subdirectory contains a different small example.

- [Hello world](1-hello/README.md)
- [`miscdev` with `read`](2-miscdev_read/README.md)
- [`miscdev` with `read` and `write`](3-miscdev/README.md)
- [Threads and spinlock](4-threads_spinlock/README.md)

