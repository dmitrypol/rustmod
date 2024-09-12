use chrono::Utc;
use cron::Schedule;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use valkey_module::Context;

lazy_static! {
    static ref CRONTAB: HashMap<&'static str, fn(&Context)> = {
        let mut map = HashMap::new();
        map.insert("*/15 * * * * * *", foo as fn(&Context));
        map.insert("*/30 * * * * * *", bar as fn(&Context));
        map
    };
}

pub(crate) fn cron_event_handler(ctx: &Context) {
    for (expression, func) in CRONTAB.iter() {
        let schedule = Schedule::from_str(expression).unwrap();
        let next_time = schedule.upcoming(Utc).next().unwrap();
        let now = Utc::now();
        // 100 milliseconds as hz 10 per second by default, look for serverCron()
        if next_time.timestamp_millis() <= now.timestamp_millis() + 100 {
            func(ctx);
        }
    }
}

fn foo(_ctx: &Context) {
    //ctx.log_notice("foo");
}

fn bar(_ctx: &Context) {
    //ctx.log_notice("bar");
}
