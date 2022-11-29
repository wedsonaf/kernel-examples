// SPDX-License-Identifier: GPL-2.0

//! Rust thread example.

use core::time::Duration;
use kernel::delay::coarse_sleep;
use kernel::prelude::*;
use kernel::sync::SpinLock;
use kernel::task::{KTask, Task};

module! {
    type: Lkp,
    name: "lkp_thread",
    license: "GPL",
}

kernel::init_static_sync! {
    static DEMO: SpinLock<u64> = 0;
}

struct Lkp {
    _thread1: KTask,
    _thread2: KTask,
}

fn thread_function(name: &str) {
    while !Task::should_stop() {
        let mut guard = DEMO.lock();
        *guard += 1;
        pr_info!("{} {}\n", name, *guard);
        drop(guard);
        coarse_sleep(Duration::from_secs(1));
    }
}

impl kernel::Module for Lkp {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        Ok(Lkp {
            _thread1: Task::spawn(fmt!("thread1"), || thread_function("thread1"))?,
            _thread2: Task::spawn(fmt!("thread2"), || thread_function("thread2"))?,
        })
    }
}
