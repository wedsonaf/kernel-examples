# Exercise 2: Readable `miscdev` module in Rust

## Overview

In this exercise you will inspect a kernel module written in Rust that registers a `miscdev` device and provides a `read` implementation.

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
    sudo insmod miscdev_read.ko
```

## Task 5: Check for presence of device
Inspect the list of `miscdev` devices:

```
    cat /proc/misc
```

You should see a messages as follows (`lkp_miscdev_read` is our device):

```
123 lkp_miscdev_read
124 cpu_dma_latency
125 vmbus/hv_kvp
126 vmbus/hv_vss
236 device-mapper
237 loop-control
144 nvram
228 hpet
235 autofs
231 snapshot
183 hw_random
127 vga_arbiter
242 rfkill
```

You can also see it in `/dev`:

```
    ls -l /dev/lkp_miscdev_read
```

As follows:

```
crw------- 1 root root 10, 123 Nov 30 14:14 /dev/lkp_miscdev_read
```

## Task 6: Read from device

Read from device using `cat`:

```
    sudo cat /dev/lkp_miscdev_read
```

You should see:

```
my file data
```

Since the file is opened before it's read, you should also inspect the system log:

```
    sudo dmeg
```

You should see a message as follows:

```
[38097.623850] lkp_miscdev_read: open on device 1
```

## Task 7: Remove the module

Remove the module using `rmmod`:

```
    sudo rmmod miscdev_read
```

## Task 8: Check device is removed

Inspect the list of `miscdev` devices:

```
    cat /proc/misc
```

You should see a messages as follows (`lkp_miscdev_read` is not there anymore):

```
124 cpu_dma_latency
125 vmbus/hv_kvp
126 vmbus/hv_vss
236 device-mapper
237 loop-control
144 nvram
228 hpet
235 autofs
231 snapshot
183 hw_random
127 vga_arbiter
242 rfkill
```

You can also try to find it in `/dev`:

```
    ls -l /dev/lkp_miscdev_read
```

You should get the following error:

```
ls: cannot access '/dev/lkp_miscdev_read': No such file or directory
```

## Summary

In this exercise, you got familiar with registering `miscdev` devices from Rust modules, and how unregistration is automatic when the module is unloaded. You also saw how a `read` implementation is exercised from the shell.