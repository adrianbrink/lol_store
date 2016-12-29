- figure out how to save foreign keys with diesel
    - for example, I should be able to save Games directly and the included vector get saved to a different table with the
    proper mapping using a foreign keys


#TODO
- deserialize the featured games and add all involved summoner ids to SummonerUniqueQueue

- implement the matchlist endpoint and store each matchId in MatchUniqueQueue
    - that are RANKED, 5v5 and in 2016

- implement the match endpoint to get data for every game
    - add involved summoner ids to UniqueQueue
    - store the results in Postgres
        - every field of type vec is a new table and every object of the game type that contains a vec creates a new row in one
        of the other tables
        - maintain a set of all matchIds that I've already downloaded

- start by writing tests for the functionality I want
- implement the RateLimiter
- implement the APIClient

#The goal:
A small server utility that downloads data from the league api and stores it in a postgres database for future analysis.

#The process:
The program contacts the league api up to the rate limit and can also work with multiple api keys to further increase the limit.
It starts with the featured games, and then stores all the data in postgres while storing the summoner id's in a queue using
redis. Then it takes the first summoner id from the queue and pulls all game data for that id. From those games it extracts the
summoner ids and adds them to the unique queue. Then it runs indefinitely.

#How to get the data
- everything just for EUW and SEASON2016
- get one summoner -> get all games with matchlist -> get data for every game with match -> extract all summoners in those games
-> repeat the process
- when redis queue is empty just check every 30 seconds, then I can manually resupply the queue
    - needed in case there is no overlap between leagues

#The design:
RateLimiter {
- an object that schedules the execution of tasks and limits the throughput
- takes a job (probably a closure) and executes it at a later point
- maintains an internal counter and timer
- only works in memory and nothing is written to disk
}

APIClient {
- contacts the league api and returns a deserialized object
}

DBDriver {
- takes a deserialized object and saves it to the database
- makes sure that a game isn't inserted more than once into the database
}

Scheduler {
- asks the APIClient to make a request and extracts the summoner ids from the response
- maintains an internal in-memory queue of summoner ids
- persist internal queue to redis using the RedisDriver in refills the queue when it runs low
- takes the next summoner id and makes a request with it using the APIClient
}

RedisDriver {
- saves a summoner id to redis
- loads a summoner id from redis
- maintain a list that acts as a queue
- maintain a set to ensure uniqueness
- when saving a new value, first store it in the set and if no error then it is unique
- if unique then store it in the list with RPUSH
- only pop items with LPOP
}

ConnectionManager {
    - stores references to all database connections
}


#Done
- implement the RedisConnector
    - redis queue:
        UniqueQueue {
            push(val) {
                let set_val = redis::set::SADD(val);
                if set_val == 1 {
                    redis::list::RPUSH(val);
                } else {
                    // do nothing
                }
            }
            pop() {
                let val = redis::list::LPOP();
                let _ = redis::set::SREM(val);
                val
            }
        }
    - implement this in my redis_connection.rs with RedisConnector