# Exercise 1: Hello world module in Rust

## Overview

In this exercise you will inspect a "Hello world" kernel module written in Rust. It writes a message to the system log when it is loaded. You will extend it to also print a message when the module is unloaded.


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

Inspect `Makefile`, the only difference from the C version is that we have `LLVM=1` to use the LLVM toolchain instead of `gcc`.

## Task 4: Load the module

Load the module using `insmod`:

```
    sudo insmod hello.ko
```

## Task 5: Check for messages from the module
Check for messages from the module using `dmesg`:

```
    sudo dmesg
```

You should see a messages as follows:

```
[ 2736.667232] hello: Hello world
```

## Task 6: Remove the module

Remove the module using `rmmod`:

```
    sudo rmmod hello
```

## Task 7: Add a `Drop` implementation

Add the following code to `hello.rs`:
```rust
impl Drop for Hello {
    fn drop(&mut self) {
        pr_info!("Goodbye world!\n");
    }
}
```

Repeat steps 3 to 6 above. After step 6, check the log again:

```
    sudo dmesg
```

You should see a message as follows:

```
[ 2742.305703] hello: Goodbye world!
```

## Summary

In this exercise, you got familiar with building kernel modules in Rust, loading and unloading them, and inspecting the system log for messages.
