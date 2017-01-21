[![](https://tokei.rs/b1/github/adrianbrink/lol_store)](https://github.com/adrianbrink/lol_store)





#The goal:
A small server utility that downloads data from the league api and stores it in a postgres
database for future analysis.


#How to run
docker-compose -f docker-compose.dev.yml up
docker build -t lol_store .
docker run --rm -it -v $(pwd):/source --network lolstore_default lol_store /bin/bash
docker run --rm -it --link lolstore_redis_1 --network lolstore_default redis bash -c 'redis-cli -h redis'

#TODO
1. write the RateLimiter
    - simple counter that easier allows the call or doesn't
2. switch to 2 redis sets for summoner_queue and match_queue
    - 1st set contains the items to visit, 2nd set contains the items already visited
    - when adding a new item, check the 2nd first and then add it
    - when removing an item, add it to the 2nd set
    - need to be atomic updates to both
3. get the integration with travis to work properly so that travis can run all my tests
4. switch to multi-threaded, run the get_featured_games() in a separate thread
    - should check after the interval again and only if the RateLimiter allows
5. switch get_matchlist() to multi-threaded
    - should grap a random item from summoner_queue and get the matches for that summoner
    - check only execute if the RateLimiter allows it
6. switch get_match() to <multi-threaded></multi-threaded>
    - should grap a random item from match_queue and get the duration and all involved summoners
    - add duration and match_id to postgres
    - add all involved summoners to summoner_queue
    - should only execute if the RateLimiter allows it
7. enable continous deployment once the server works
    - every push to development should trigger a build and on success it should merge it into master
    - every update to master causes the production environment to change and deploy the new code
    - https://circleci.com/docs/docker/#application-deployment


# Potential development workflow
- in the morning run `docker-compose up` and then connect to the running shell
- then write some code, go the shell with the container and run `cargo build`
- be happy with the changes and in a normal shell commit everything and then push it
to development
- CircleCI gets triggered and runs `cargo test`, if successful it merges it into
master

** Outdated **
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
- deserialize the featured games and add all involved summoner ids to SummonerUniqueQueue
- implement the matchlist endpoint and store each matchId in MatchUniqueQueue
    - that are RANKED, 5v5 and in 2016
- dockerizing the application so that a simple docker-compose allows you to create all
needed other containers, as well as docker run for easy development
- integration with CircleCI works to run tests
