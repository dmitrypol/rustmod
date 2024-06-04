#[macro_use]
extern crate valkey_module;

use valkey_module::{alloc::ValkeyAlloc, Context, Status, ValkeyResult, ValkeyString, ValkeyValue};

fn initialize(ctx: &Context, _args: &[ValkeyString]) -> Status {
    ctx.log_notice("initialize rustmod");
    Status::Ok
}

fn deinitialize(ctx: &Context) -> Status {
    ctx.log_notice("deinitialize rustmod");
    Status::Ok
}

fn hello(_ctx: &Context, _args: Vec<ValkeyString>) -> ValkeyResult {
    Ok(ValkeyValue::SimpleStringStatic("world"))
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
    ]
}
