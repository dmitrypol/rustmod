use anyhow::{Context, Result};
mod utils;

#[test]
fn test_suite() -> Result<()> {
    let _guards =
        vec![utils::start_valkey_server_with_module().with_context(|| "failed to start")?];
    let mut con = utils::get_valkey_connection().with_context(|| "failed to connect")?;

    // test hello - no args, default to world
    let resp_hello: String = redis::cmd("rustmod.hello")
        .query(&mut con)
        .with_context(|| "failed to run hello")?;
    assert_eq!(resp_hello, "world");

    // test hello - specify arg
    let resp_hello: String = redis::cmd("rustmod.hello")
        .arg("foo")
        .query(&mut con)
        .with_context(|| "failed to run hello")?;
    assert_eq!(resp_hello, "foo");

    // test hello2
    let resp_hello: String = redis::cmd("rustmod.hello2")
        .query(&mut con)
        .with_context(|| "failed to run hello2")?;
    assert_eq!(resp_hello, "world");

    // test hello3
    let resp_hello: String = redis::cmd("rustmod.hello3")
        .arg("foo")
        .query(&mut con)
        .with_context(|| "failed to run hello3")?;
    assert_eq!(resp_hello, "foo");

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

    // test myset
    let resp_myset: String = redis::cmd("rustmod.myset")
        .arg(&["key3", "value3"])
        .query(&mut con)
        .with_context(|| "failed to run myset")?;
    assert_eq!(resp_myset, "OK");

    // test myget
    let resp_myget: String = redis::cmd("rustmod.myget")
        .arg(&["key3"])
        .query(&mut con)
        .with_context(|| "failed to run myget")?;
    assert_eq!(resp_myget, "value3");

    Ok(())
}
