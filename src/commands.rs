use crate::data_types::{MyDataType, MY_DATA_TYPE};
use crate::*;
use std::collections::BTreeMap;
use std::sync::atomic::Ordering::SeqCst;
use valkey_module::{Context, NextArg, ValkeyError, ValkeyResult, ValkeyString, ValkeyValue};

pub(crate) fn hello(_ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    let resp = if args.len() == 1 {
        ValkeyValue::SimpleStringStatic("world")
    } else {
        ValkeyValue::SimpleStringStatic(args.into_iter().skip(1).next_str()?)
    };
    Ok(resp)
}

pub(crate) fn hello2(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    hello(ctx, args)
}

/// using high level call to perform write and read
pub(crate) fn setget(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    if args.len() != 3 {
        return Err(ValkeyError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);
    let key = args.next_str()?;
    let value = args.next_str()?;
    // write operation
    let resp_set = ctx.call("set", &[key, value])?;
    ctx.log_notice(&format!("resp_set: {:?}", resp_set));
    // read operation
    let resp_get = ctx.call("get", &[key])?;
    ctx.log_notice(&format!("resp_get: {:?}", resp_get));
    Ok(resp_get)
}

// using low level key operations to perform write and read
pub(crate) fn setget2(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    if args.len() != 3 {
        return Err(ValkeyError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);
    let key_name = args.next_arg()?;
    let value = args.next_arg()?;
    // write operation
    let key_writable = ctx.open_key_writable(&key_name);
    let mut key_dma = key_writable.as_string_dma()?;
    let resp_write = key_dma
        .write(value.as_slice())
        .map(|_| ValkeyValue::SimpleStringStatic(""));
    ctx.log_notice(&format!("resp_write: {:?}", resp_write));
    // read operation
    let key = ctx.open_key(&key_name);
    let resp_read = key.read()?.map_or(ValkeyValue::Null, |v| {
        ValkeyValue::StringBuffer(Vec::from(v))
    });
    ctx.log_notice(&format!("resp_read: {:?}", resp_read));
    Ok(resp_read)
}

pub(crate) fn myset(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    if args.len() != 3 {
        return Err(ValkeyError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let value = args.next_arg()?;
    let key2 = ctx.open_key_writable(&key);
    let value2 = MyDataType { data: value.into() };
    key2.set_value(&MY_DATA_TYPE, value2)?;
    Ok("OK".into())
}

pub(crate) fn myget(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    if args.len() != 2 {
        return Err(ValkeyError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let key2 = ctx.open_key(&key);
    let value = match key2.get_value::<MyDataType>(&MY_DATA_TYPE)? {
        Some(value) => value.data.as_str().into(),
        None => ().into(),
    };
    Ok(value)
}

pub(crate) fn config(_ctx: &Context, _args: Vec<ValkeyString>) -> ValkeyResult {
    let mut output = BTreeMap::new();
    unsafe {
        output.insert(
            "CONFIG_STR".into(),
            CONFIG_STR.clone().unwrap_or_default().into(),
        );
        output.insert("CONFIG_BOOL".into(), CONFIG_BOOL.load(SeqCst).into());
        output.insert("CONFIG_NUM".into(), CONFIG_NUM.load(SeqCst).into());
    }
    Ok(ValkeyValue::OrderedMap(output))
}

pub(crate) fn set_metadata(_ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    if args.len() != 3 {
        return Err(ValkeyError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let value = args.next_string()?;
    unsafe {
        let mut tmp = METADATA.clone().unwrap();
        tmp.insert(key, value);
        METADATA = Some(tmp);
    }
    Ok("OK".into())
}

pub(crate) fn get_metadata(_ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    if args.len() != 1 {
        return Err(ValkeyError::WrongArity);
    }
    let metadata = unsafe { METADATA.as_ref().unwrap() };
    let mut output = BTreeMap::new();
    for (k, v) in metadata.iter() {
        output.insert(k.clone(), v.clone());
    }
    Ok(output.into())
}
