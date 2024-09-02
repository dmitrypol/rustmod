use anyhow::{Context, Result};
mod utils;

#[test]
fn test_suite() -> Result<()> {
    let _guards =
        vec![utils::start_valkey_server_with_module().with_context(|| "failed to start")?];
    let mut con = utils::get_valkey_connection().with_context(|| "failed to connect")?;

    // test hello
    let resp_hello: String = redis::cmd("rustmod.hello")
        .query(&mut con)
        .with_context(|| "failed to run hello")?;
    assert_eq!(resp_hello, "world");

    // test setget
    let resp_setget: String = redis::cmd("rustmod.setget")
        .arg(&["key", "value"])
        .query(&mut con)
        .with_context(|| "failed to run setget")?;
    assert_eq!(resp_setget, "value");

    // test setget2
    let resp_setget2: String = redis::cmd("rustmod.setget2")
        .arg(&["key2", "value2"])
        .query(&mut con)
        .with_context(|| "failed to run setget2")?;
    assert_eq!(resp_setget2, "value2");

    Ok(())
}
