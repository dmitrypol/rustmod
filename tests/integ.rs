use anyhow::{Context, Result};
use redis::RedisError;

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

    // test config
    // [bulk-string('"CONFIG_BOOL"'), int(0), bulk-string('"CONFIG_NUM"'), int(0), bulk-string('"CONFIG_STR"'), bulk-string('""')])
    let resp_config: [String; 6] = redis::cmd("rustmod.config")
        .query(&mut con)
        .with_context(|| "failed to run config")?;
    assert_eq!(resp_config.len(), 6);

    // test set_metadata
    let resp_set_metadata: String = redis::cmd("rustmod.set_metadata")
        .arg(&["key1", "value1"])
        .query(&mut con)
        .with_context(|| "failed to run set_metadata")?;
    assert_eq!(resp_set_metadata, "OK");

    // test set_metadata WrongArity
    let resp_set_metadata_no_args: Result<String, RedisError> =
        redis::cmd("rustmod.set_metadata").query(&mut con);
    if resp_set_metadata_no_args.is_err() {
        assert_eq!(
            resp_set_metadata_no_args.err().unwrap().to_string(),
            "An error was signalled by the server - ResponseError: wrong number of arguments for 'rustmod.set_metadata' command"
        );
    }

    // test get_metadata
    // array([bulk-string('"key1"'), bulk-string('"value1"')])
    let resp_get_metadata: [String; 2] = redis::cmd("rustmod.get_metadata")
        .query(&mut con)
        .with_context(|| "failed to run get_metadata")?;
    assert_eq!(resp_get_metadata.len(), 2);

    // test get_metadata WrongArity
    let resp_get_metadata_w_args: Result<String, RedisError> = redis::cmd("rustmod.get_metadata")
        .arg(&["invalid"])
        .query(&mut con);
    if resp_get_metadata_w_args.is_err() {
        assert_eq!(
            resp_get_metadata_w_args.err().unwrap().to_string(),
            "An error was signalled by the server - ResponseError: wrong number of arguments for 'rustmod.get_metadata' command"
        );
    }

    Ok(())
}
