# Exercise 4: Working with threads an spinlocks

## Overview

In this exercise you will inspect sample code in a Rust module that utilizes spinlocks. The module prints messages to the system log. It spawns two kernel threads and each one outputs a line into the log once per second.



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
    sudo insmod thread.ko
```

## Task 5: Check for message from the module

Use `dmesg` to read messages from the log:

```
    sudo dmesg
```

You should see messages such as follows:

```
[42136.100706] lkp_thread: thread1 1
[42136.100775] lkp_thread: thread2 2
[42137.163354] lkp_thread: thread1 3
[42137.163447] lkp_thread: thread2 4
[42138.187395] lkp_thread: thread1 5
[42138.187411] lkp_thread: thread2 6
[42139.211225] lkp_thread: thread1 7
[42139.211339] lkp_thread: thread2 8
[42140.235283] lkp_thread: thread2 9
[42140.235357] lkp_thread: thread1 10
[42141.259154] lkp_thread: thread2 11
[42141.259167] lkp_thread: thread1 12
[42142.283155] lkp_thread: thread2 13
```

## Task 6: Remove the module

Afer several seconds, remove the module to avoid overfilling disk and logs:

```
    sudo rmmod thread
```


## Summary

In this exercise, you got familiar with usage of threads and spinlocks. You inspected code illustrating an example of using spinlocks.
