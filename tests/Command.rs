use rust_redis_client::redis_client::RedisClient;
#[test]
fn SendsAndGetsResponseForPing() -> Result<(), std::io::Error> {
    let uri = "127.0.0.1:6379";
    let client = RedisClient::new(uri);
    let resp = client.send_cmd(vec!["PING"])?;
    assert_eq!(resp, "+PONG");
    Ok(())
}
