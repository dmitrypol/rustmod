mod commands;
mod data_types;
mod event_handlers;
mod my_cron;

#[macro_use]
extern crate valkey_module;

use commands::*;
use event_handlers::*;
use std::collections::HashMap;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicI64};
use valkey_module::{alloc::ValkeyAlloc, Context, Status, ValkeyString};

static mut CONFIG_STR: Option<String> = None;
static CONFIG_BOOL: AtomicBool = AtomicBool::new(false);
static CONFIG_NUM: AtomicI64 = AtomicI64::new(0);
static mut METADATA: Option<HashMap<String, String>> = None;

fn initialize(ctx: &Context, args: &[ValkeyString]) -> Status {
    let mut args = args.iter();
    unsafe {
        METADATA = Some(HashMap::new());
    }
    let (config_str, config_bool, config_num) = unsafe {
        CONFIG_STR.replace(args.next().unwrap_or(&ctx.create_string("")).to_string());
        CONFIG_BOOL.store(
            args.next().unwrap_or(&ctx.create_string("")).to_string() == "true",
            SeqCst,
        );
        CONFIG_NUM.store(
            args.next()
                .unwrap_or(&ctx.create_string(""))
                .to_string()
                .parse::<i64>()
                .unwrap_or(0),
            SeqCst,
        );
        (
            CONFIG_STR.clone(),
            CONFIG_BOOL.load(SeqCst),
            CONFIG_NUM.load(SeqCst),
        )
    };
    ctx.log_notice(
        format!("initialized with: {config_str:?} {config_bool:?} {config_num:?}").as_str(),
    );
    Status::Ok
}

fn deinitialize(ctx: &Context) -> Status {
    ctx.log_notice("deinitialize");
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
        ["rustmod.hello", hello, "readonly", 0, 0, 0],
        ["rustmod.hello2", hello2, "readonly", 0, 0, 0],
        ["rustmod.hello3", hello, "readonly", 0, 0, 0],
        ["rustmod.setget", setget, "write", 0, 0, 0],
        ["rustmod.setget2", setget2, "write", 0, 0, 0],
        ["rustmod.myset", myset, "write", 0, 0, 0],
        ["rustmod.myget", myget, "readonly", 0, 0, 0],
        ["rustmod.config", config, "readonly", 0, 0, 0],
        ["rustmod.set_metadata", set_metadata, "write", 0, 0, 0],
        ["rustmod.get_metadata", get_metadata, "readonly", 0, 0, 0],
    ]
    event_handlers: [
        [@ALL: event_handler_all],
    ],
}
