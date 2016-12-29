extern crate lol_store;

use lol_store::redis_connection::{RedisConnector, UniqueQueue};

#[cfg(test)]
mod unique_queue {
    use super::*;
    fn new_unique_queue() -> UniqueQueue {
        let redis = RedisConnector::new().unwrap();
        let queue = UniqueQueue::new(redis.connection, "summoner_id".to_string());
        queue
    }

    #[test]
    fn push_value_onto_queue() {
        let queue = new_unique_queue();
        queue.push("1".to_string());
        // assert_eq!("1".to_string(), queue.pop());
    }

    #[test]
    #[ignore]
    fn pop_value_from_queue() {
        let queue = new_unique_queue();
        queue.push("1".to_string());
        assert_eq!(queue.pop(), "1".to_string());
    }
}

#[cfg(test)]
mod redis_connector {
    use super::*;
    #[test]
    fn new_redis_connector() {
        let redis = RedisConnector::new().unwrap();
        assert!(true);
    }
}