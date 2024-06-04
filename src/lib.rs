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

/// simple command that returns a static string
fn hello(_ctx: &Context, _args: Vec<ValkeyString>) -> ValkeyResult {
    Ok(ValkeyValue::SimpleStringStatic("world"))
}

/// using high level call to perform write and read
fn setget(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    if args.len() < 3 {
        return Err(ValkeyError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);
    let key = args.next_str()?;
    let value = args.next_str()?;
    // write operation
    let _ = ctx.call("set", &[key, value]);
    // read operation
    let resp = ctx.call("get", &[key])?;
    ctx.log_notice(&format!("resp: {:?}", resp));
    Ok(resp)
}

// using low level key operations to perform write and read
fn setget2(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    if args.len() < 3 {
        return Err(ValkeyError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);
    let key_name = args.next_arg()?;
    let value = args.next_arg()?;
    // write operation
    let key_writable = ctx.open_key_writable(&key_name);
    let mut key_dma = key_writable.as_string_dma()?;
    let _ = key_dma
        .write(value.as_slice())
        .map(|_| ValkeyValue::SimpleStringStatic(""));
    // read operation
    let key = ctx.open_key(&key_name);
    let resp = key.read()?.map_or(ValkeyValue::Null, |v| {
        ValkeyValue::StringBuffer(Vec::from(v))
    });
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
        ["rustmod.setget", setget, "write", 0, 0, 0],
        ["rustmod.setget2", setget2, "write", 0, 0, 0],
    ]
}
