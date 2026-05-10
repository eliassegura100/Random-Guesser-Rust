// SPDX-License-Identifier: GPL-2.0
//! rustguess: minimal Rust kernel module smoke test.

use kernel::prelude::*;

module! {
    type: HelloModule,
    name: "rustguess",
    authors: ["Elias Segura"],
    description: "rustguess — a number-guessing game character device",
    license: "GPL",
}

struct HelloModule;

impl kernel::Module for HelloModule {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("module loaded\n");
        Ok(HelloModule)
    }
}

impl Drop for HelloModule {
    fn drop(&mut self) {
        pr_info!("module unloaded\n");
    }
}
