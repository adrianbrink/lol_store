use dotenv::dotenv;
use std::env;
use redis::{Client, Commands, Connection, RedisError};

pub struct RedisConnector {
    connection: Connection,
}

impl RedisConnector {
    pub fn new() -> Result<RedisConnector, RedisConnectorError> {
        dotenv().ok();
        //let url = env::var("redis://redis/")?;
        let client = Client::open("redis://redis/")?;
        let conn = client.get_connection()?;
        Ok(RedisConnector { connection: conn })
    }

    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }
}

#[derive(Debug)]
pub enum RedisConnectorError {
    Var(env::VarError),
    Redis(RedisError),
}

impl From<env::VarError> for RedisConnectorError {
    fn from(err: env::VarError) -> RedisConnectorError {
        RedisConnectorError::Var(err)
    }
}

impl From<RedisError> for RedisConnectorError {
    fn from(err: RedisError) -> RedisConnectorError {
        RedisConnectorError::Redis(err)
    }
}
