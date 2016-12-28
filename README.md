#TODO
- write unit tests for all existing public functions
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