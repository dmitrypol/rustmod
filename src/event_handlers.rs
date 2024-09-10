use crate::my_cron;
use valkey_module::server_events::LoadingSubevent;
use valkey_module::{Context, NotifyEvent};
use valkey_module_macros::*;

#[loading_event_handler]
fn loading_event_handler(ctx: &Context, _values: LoadingSubevent) {
    ctx.log_notice("loading_event_handler");
}

#[config_changed_event_handler]
fn config_changed_event_handler(ctx: &Context, values: &[&str]) {
    ctx.log_notice(&format!("config_changed_event_handler: {:?}", values));
}

#[cron_event_handler]
fn cron_event_handler(ctx: &Context, _hz: u64) {
    my_cron::cron_event_handler(ctx);
}

pub(crate) fn event_handler_all(ctx: &Context, event_type: NotifyEvent, event: &str, key: &[u8]) {
    ctx.log_notice(&format!(
        "event_handler_all: {:?} {:?} {:?}",
        event_type, event, key
    ));
}
