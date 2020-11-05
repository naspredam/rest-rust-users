# rest-rust-users

This project has as objective to have a rest api build on:

- Rust (nightly) 1.49.0-nightly
- Diesel
- Rocket
- MySQL (8.0.22)

Using standard rest/web strategy, non-reactive.

## What the rest api stands for

The rest will have the resource:

```vim
/users
```

Where the endpoints exposed are:

| Method | Endpoint | Description  |
| ---    |:------- |:-----|
|GET| /users | Get all the users |
|POST| /users | Create a new user |
|PUT| /users/{user_id} | Update specific user data |
|GET| /users/{user_id} | Get specific user data |
|DELETE| /users/{user_id} | Delete specific user data |

## Run application

This project has been set to run under docker.

To run the application the `Makefile` has been set to:

- start: to start the application in docker
- stop: stop and drop the containers
- restart: does stop and start
- logs: display the logs of the deployed docker containers

So, the `start` action will do:

- build the image, which will compile the code as a builder inside of a rust docker image
- start the containers of the database (mysql) and service

The `stop` action will do:

- stop the application and database
- destroy the containers for the application and the database
