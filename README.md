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
- [`403`](http://0.0.0.0:8000/status/403) not login but action needs to
- [`404`](http://0.0.0.0:8000/status/404) not found
- [`405`](http://0.0.0.0:8000/status/405) try to do something only accessible for visitors
- [`417`](http://0.0.0.0:8000/status/417) code should not happen, it's when the user of the token doesn't exist
- [`418`](http://0.0.0.0:8000/status/418) expired token
- [`500`](http://0.0.0.0:8000/status/500) error internal (my bad)

## Home
- `GET` : [`/`](http://0.0.0.0:8000/) Home of the website
- `GET` : [`/status/code`](http://0.0.0.0:8000/status/<code>) With `<code>` a response status code 
  allows to simulate a code to see the template to it 

## Account 
*First the server will always have an admin account 
with a default password `password` (you can change it)*
### Path 
- `GET` : [`/account/home`](http://0.0.0.0:8000/account/home) Send to login if not login else display user personal page
- `GET` : [`/account/users`](http://0.0.0.0:8000/account/users) Allows to see every member of the website (if you're connected as admin get state of the database)
- `GET` : [`/account/register`](http://0.0.0.0:8000/account/register) Show a form to fill to create a new account
- `POST` : `/account/register` Handle the form and try to create the account
- `GET` : [`/account/users`](http://0.0.0.0:8000/account/users) Show a form to fill to login 
- `POST` : `/account/login` Handle the form and try to login
- `PUT` : `/account/logout` Disconnect the user
- `DELETE` : `/account/delete` Try to delete the user
- `GET` : [`/account/edit`](http://0.0.0.0:8000/account/edit) Show form to change password or profile picture
- `POST` : `/account/edit` Handle the change password
- `POST` : `/account/set/picture` Handle the new picture

--- 

## To do
- better check form
- edit
