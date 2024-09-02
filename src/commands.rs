use valkey_module::{Context, NextArg, ValkeyError, ValkeyResult, ValkeyString, ValkeyValue};

pub(crate) fn hello(_ctx: &Context, _args: Vec<ValkeyString>) -> ValkeyResult {
    Ok(ValkeyValue::SimpleStringStatic("world"))
}

/// using high level call to perform write and read
pub(crate) fn setget(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
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
pub(crate) fn setget2(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
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
