use redis::{Commands, Connection};

pub struct UniqueQueue<'a> {
    connection: &'a Connection,
    key_name: String,
    set_name: String,
    list_name: String,
}

impl<'a> UniqueQueue<'a> {
    pub fn new(connection: &'a Connection, name: String) -> UniqueQueue {
        let set = format!("{}_set", &name);
        let list = format!("{}_list", &name);
        UniqueQueue {
            connection: connection,
            key_name: name,
            set_name: set,
            list_name: list,
        }
    }

    // This should panic when the Result from adding to the set is Err,
    // because it implies that the connection isn't set up properly.
    // TODO - make this an atomic operation
    pub fn push(&self, value: i64) -> i64 {
        let x = self.connection.sadd::<_, _, i64>(&self.set_name, value).unwrap();
        if x == 1 {
            self.connection.rpush::<_, _, i64>(&self.list_name, value).unwrap();
        }
        x
    }

    // TODO - make this an atomic operation
    pub fn pop(&self) -> Option<i64> {
        let x = self.connection.lpop::<_, i64>(&self.list_name);
        match x {
            Ok(val) => {
                let _ = self.connection.srem::<_, _, i64>(&self.set_name, val).unwrap();
                Some(val)
            }
            Err(_) => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        let length: i64 = self.connection
            .llen(&self.list_name)
            .expect("Getting the length of the list in redis failed.");
        if length > 0 { false } else { true }
    }
}
