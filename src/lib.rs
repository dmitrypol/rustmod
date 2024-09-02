mod commands;

#[macro_use]
extern crate valkey_module;

use commands::*;
use valkey_module::{alloc::ValkeyAlloc, Context, Status, ValkeyString};

fn initialize(ctx: &Context, _args: &[ValkeyString]) -> Status {
    ctx.log_notice("initialize rustmod");
    Status::Ok
}

fn deinitialize(ctx: &Context) -> Status {
    ctx.log_notice("deinitialize rustmod");
    Status::Ok
}

valkey_module! {
    name: "rustmod",
    version: 1,
    allocator: (ValkeyAlloc, ValkeyAlloc),
    data_types: [],
    init: initialize,
    deinit: deinitialize,
    commands: [
        ["rustmod.hello", hello, "", 0, 0, 0],
        ["rustmod.setget", setget, "write", 0, 0, 0],
        ["rustmod.setget2", setget2, "write", 0, 0, 0],
    ]
}
