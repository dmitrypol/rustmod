use std::os::raw::c_void;
use valkey_module::{native_types::ValkeyType, raw};

#[derive(Debug)]
pub(crate) struct MyDataType {
    pub(crate) data: String,
}

pub(crate) static MY_DATA_TYPE: ValkeyType = ValkeyType::new(
    "mydatatyp",
    0,
    raw::RedisModuleTypeMethods {
        version: raw::REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: Some(rdb_load),
        rdb_save: Some(rdb_save),
        aof_rewrite: None,
        mem_usage: None,
        digest: None,
        free: Some(free),
        aux_load: None,
        aux_save: None,
        aux_save2: None,
        aux_save_triggers: 0,
        free_effort: None,
        unlink: None,
        copy: None,
        defrag: None,
        mem_usage2: None,
        free_effort2: None,
        unlink2: None,
        copy2: None,
    },
);

// callbacks

unsafe extern "C" fn free(value: *mut c_void) {
    drop(Box::from_raw(value.cast::<MyDataType>()));
}

unsafe extern "C" fn rdb_save(rdb: *mut raw::RedisModuleIO, value: *mut c_void) {
    let value = &*value.cast::<MyDataType>();
    raw::save_string(rdb, value.data.as_str());
}

unsafe extern "C" fn rdb_load(rdb: *mut raw::RedisModuleIO, _encver: i32) -> *mut c_void {
    let data = raw::load_string(rdb);
    let data = match data {
        Ok(data) => data,
        Err(_) => return std::ptr::null_mut(),
    };
    let value = MyDataType {
        data: data.to_string(),
    };
    Box::into_raw(Box::new(value)) as *mut c_void
}
