# Installation 
Install [Docker](https://docs.docker.com/engine/installation/) & [Docker-Compose](https://docs.docker.com/compose/install/)

Then download the repository go to the root where the 
`Dockerfile` is and do: `sudo docker-compose up` 
*sudo is needed to access to the port*

In the terminal where you launch the docker-compose you should see 
something like that if all work : 
`
Rocket has launched from http://0.0.0.0:8000
`
Just go in the direction given on the terminal or go [Here](http://0.0.0.0:8000)
If it's not working then try :
- `localhost:8000`
- `{docker ip}:8000`
- `http://0.0.0.0:8000`

# Closing
When the server is running to close properly,
you just need to `ctrl c` and run `sudo docker-compose down`

# Tests
**To see if the server passes all the tests you need to:**

go to the `root` of the project and do: 
- `sudo docker-compose up -d` to launch the container in daemon mode
- `sudo docker container ls`
You should see something like this 
```docker
CONTAINER ID   IMAGE                   COMMAND                  CREATED          STATUS         PORTS                                       NAMES
1bea5cd8857b   to_do_reminder_server   "bash -c 'cargo run …"   19 minutes ago   Up 6 minutes   0.0.0.0:8000->8000/tcp, :::8000->8000/tcp   to_do_reminder_server_1
```
- copy the id under `CONTAINER ID`
- `sudo docker container exec CONTAINERID cargo test`
    replace `CONTAINERID` by the id of your container 
  - launch the test and hopefully everything will be ok
- Dont forget to do `sudo docker-compose down` once you're done  
--- 

# Implemented
## Status code
**You can try every status code by going to [{server}/status/\<code>](http://0.0.0.0:8000/status/404)**
- `403` not login but action needs to
- `404` not found
- `405` try to do something only accessible for visitors
- `417` code should not happen, it's when the user of the token doesn't exist
- `418` expired token
- `500` error internal (my bad)

--- 

## To do
- ask user confirmation javascript?
- user_display
- home path in account
- test section account
