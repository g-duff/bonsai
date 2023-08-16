use redis;
use redis::{Client, RedisResult};

fn main() -> RedisResult<()> {
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let mut pubsub = con.as_pubsub();
    pubsub.subscribe("channel_1")?;
    pubsub.subscribe("channel_2")?;

    loop {
        let msg = pubsub.get_message()?;
        let payload: String = msg.get_payload()?;
        println!("channel '{}': {}", msg.get_channel_name(), payload);
    }
}
