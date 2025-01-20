use redis::{Connection, RedisResult};

pub fn connect_to_redis() -> RedisResult<Connection> {
    let client = redis::Client::open("redis://127.0.0.1")?;
    let connection = client.get_connection();
    return connection;
}
