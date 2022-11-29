# Exercise 3: Readable and writable `miscdev` module in Rust

## Overview

In this exercise you will inspect an update to the previous module where we also have a `write` implementation for the `miscdev` device.

## Task 1: Connect to your lab VM

1. Connect to your VM using _ssh_.

## Task 2: Setup _tmux_

Start _tmux_ so that connection issues do not prevent you from being able to get back to your shell:
```
    tmux -u attach -d || tmux -u
```

## Task 3: Build the module
Build the module with `make`:
```
    make
```

## Task 4: Load the module

Load the module using `insmod`:

```
    sudo insmod miscdev.ko
```

## Task 5: Check for presence of device

You can also see it in `/dev`:

```
    ls -l /dev/lkp_miscdev
```

As follows:

```
crw-rw-rw- 1 root root 10, 123 Nov 30 14:53 /dev/lkp_miscdev
```

Note that the permissions are different: now everyone is allowed to read from and write to the device.

## Task 6: Write to device

Write to device using redirection:

```
    echo testing > /dev/lkp_miscdev
```

## Task 7: Read from device

Read from device using `cat`:

```
    cat /dev/lkp_miscdev
```

You should see:

```
testing
```

This is the text that was written in the previous task. You can repeat tasks 6 and 7 with different contents.

## Challenge: Several writes

If we issue multiple writes to the device (e.g., using `cat > /dev/lkp_miscdev` and writing multiple lines), `read` will only return the last one. Why? How can we fix this?

## Summary

In this exercise, you built on previous one and got familiar with how Rust ensures that memory remains available when in use, and how it forces the use of synchronization primitives when needed. You also saw how a `write` implementation can be exercised from the shell.