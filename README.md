[![](https://tokei.rs/b1/github/adrianbrink/lol_store)](https://github.com/adrianbrink/lol_store)

#The goal:
A small server utility that downloads data from the league api and stores it in a postgres
database for future analysis.

#How to run
docker-compose up
docker build -t lol_store .
docker run --rm -it -v $(pwd):/source --network lolstore_default lol_store /bin/bash

#TODO
1. dockerize the entire application, so that I can just say 'docker compose up' to run it
    - start by creating a new test application that only uses redis, postgres and prints to
    the console
    - use a shell script to configure the build environment, it should run `diesel migration run`
    and then `cargo run`
        - this should be the last command in the Dockerfile
    - then figure out how to do automatic deploys to a server on every merge to master
    - Make it print out hello over and over again and confirm that by connecting to the docker box
    running in production.
    - ensure that codeship can run all my tests and otherwise it doesn't allow me to merge
    Problems:
        - switch to docker entries in /etc/hosts to connect to the containers
        - research how migrations are run
        - CircleCI should run 'cargo test' and only then build and 
2. write the RateLimiter
    - simple counter that easier allows the call or doesn't
3. switch to 2 redis sets for summoner_queue and match_queue
    - 1st set contains the items to visit, 2nd set contains the items already visited
    - when adding a new item, check the 2nd first and then add it
    - when removing an item, add it to the 2nd set
    - need to be atomic updates to both
3. switch to multi-threaded, run the get_featured_games() in a separate thread
    - should check after the interval again and only if the RateLimiter allows
4. switch get_matchlist() to multi-threaded
    - should grap a random item from summoner_queue and get the matches for that summoner
    - check only execute if the RateLimiter allows it
5. switch get_match() to <multi-threaded></multi-threaded>
    - should grap a random item from match_queue and get the duration and all involved summoners
    - add duration and match_id to postgres
    - add all involved summoners to summoner_queue
    - should only execute if the RateLimiter allows it


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
