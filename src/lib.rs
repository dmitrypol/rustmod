#[macro_use]
extern crate valkey_module;

use valkey_module::{
    alloc::ValkeyAlloc, Context, NextArg, Status, ValkeyError, ValkeyResult, ValkeyString,
    ValkeyValue,
};

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

fn setget(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    if args.len() < 3 {
        return Err(ValkeyError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);
    let key = args.next_str().unwrap();
    let value = args.next_str().unwrap();
    // write
    let _ = ctx.call("set", &[key, value]);
    // read
    let resp = ctx.call("get", &[key])?;
    ctx.log_notice(&format!("resp: {:?}", resp));
    Ok(resp)
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
        ["rustmod.setget", setget, "", 0, 0, 0],
    ]
}
