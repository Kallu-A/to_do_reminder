# Installation 
Install [Docker](https://docs.docker.com/engine/installation/) & [Docker-Compose](https://docs.docker.com/compose/install/)

### Init the SMTP server 
Before launch the server, you need to connect an `SMTP` account if you don't have one you can by `example use gmail`

go to the `.env` file and change all the value with `SMTP`
- `ADRESS_SMTP` *It's the adresse email you using*
- `PASSWORD_SMTP` *It's your password*
- `RELAY_SMTP` *It's your relay, example if you use a gmail account, it's will be smtp.gmail.com*

*Of course you're not obligated to use a gmail account*
**If your values SMTP are corrects it's possible the link still don't work because you have the connexion refused
you just need to change in your account the setting `Allow applications with less security`**

if something with the SMTP goes wrong the server will not launch so if the server is launch or pass the test don't worry it's working

### Launch

Go to the root where the 
`docker-compose` is and do: `sudo docker-compose up` 
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

# Mode of launching 
You can use `sudo docker-compose up` and have different behavior 
at the beginning you should see something like that : 
```
server_1  | You can change the mode by changing the value of LAUNCH_MODE in the .env file
server_1  | 'r' -> release
server_1  | 'a' -> all (fmt, clippy, test, release)
server_1  | 'd' -> debug
server_1  | 't' -> test
server_1  | 'c' -> clippy (upgrade code)
server_1  | 'f' -> fmt (syntax format)
server_1  | 'rd' -> recreate the database
server_1  | Link to the database: success
server_1  | Mode is: all verif 'a'
```
Everything has been explain you just need to go to the `server/.env` file and change the value of `LAUNCH_MODE` to the mode you want
- `'r' release` launch the test before to make sure everything should work: better performance in runtime, `log level is critical`, so only important message are displayed in the terminal
- `'d' debug` use for full data display to the terminal `log level normal`
- `'t' test` launch the test of the server
- `'c' clippy` launch the clippy functionality of cargo *(test if their better syntax solution)*
- `'f' fmt` launch the fmt functionality of cargo *(reformat the code with the rust convention)*
- `'rd' diesel redo` recreate the database *(erase all data)* if you change the structure and also reset the value in the `data.json`
- `'a' all` launch fmt, clippy, test, and server in release use this before a commit to make sure everything works fine
# Tests
### Option 1
**To see if the server passes all the tests you need to:**
You can change the in the `server/.env` file the value of `LAUNCH_MODE` to `t` and do `sudo docker-compose up`

### Option 2

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

# Security 
- The password is saved in the database with the encryption `PBKDF2`
- token are generated by serialise and `sha256` encryption plus a private cookie with `base64` hash
the token and as a `expire_date of 2 hours` and a cookie to set the expired_token of `12 hours` after that the server will no longer send `expired token` but
`forbidden` status


--- 

# Implemented
## Status code
**You can try every status code by going to [{server}/status/\<code>](http://0.0.0.0:8000/status/404)**
- [`401`](http://0.0.0.0:8000/status/401) only for admin
- [`403`](http://0.0.0.0:8000/status/403) not login but action needs to
- [`404`](http://0.0.0.0:8000/status/404) not found
- [`405`](http://0.0.0.0:8000/status/405) try to do something only accessible for visitors
- [`417`](http://0.0.0.0:8000/status/417) code should not happen, it's when the user of the token doesn't exist
- [`418`](http://0.0.0.0:8000/status/418) expired token
- [`500`](http://0.0.0.0:8000/status/500) error internal (my bad)


## Account 
*First the server will always have an admin account 
with a default password `password` (you can change it) and email value same as the smtp email*

# Path 

### Home
- `GET` : [`/`](http://0.0.0.0:8000/) Home of the website
- `GET` : [`/status/code`](http://0.0.0.0:8000/status/<code>) With `<code>` a response status code
  allows to simulate a code to see the template to it

### Account
- `GET` : [`/account/home`](http://0.0.0.0:8000/account/home) Send to login if not login else display user personal page
- `GET` : [`/account/users`](http://0.0.0.0:8000/account/users) Allows to see every member of the website (if you're connected as admin get state of the database)
- `GET` : [`/account/register`](http://0.0.0.0:8000/account/register) Show a form to fill to create a new account
- `POST` : `/account/register` Handle the form and try to create the account
- `GET` : [`/account/users`](http://0.0.0.0:8000/account/users) Show a form to fill to login 
- `POST` : `/account/login` Handle the form and try to login
- `PUT` : `/account/logout` Disconnect the user
- `DELETE` : `/account/delete` Try to delete the user
- `DELETE` : `account/delete_admin/<id>` for admin account delete the username put
- `GET` : [`/account/edit`](http://0.0.0.0:8000/account/edit) Show form to change password or profile picture
- `POST` : `/account/edit` Handle the change password
- `delete` : `/account/edit/remove_picture/<id>` allow to remove the picture
- `POST` : `/account/set/picture` Handle the new picture
- `PUT` :  `/account/send_code` send a code to confirm the email
- `POST` : `/account/confirm` form to enable the email 
- `GET` : [`/account/code_password`](http://0.0.0.0:8000/account/code_password) form when you forget your password
- `PUT` : `/account/code_password` change the password and send the new value per email 
- `PUT` : `/account/new_email` change the email and send a confirm code to enable the email

### To-Do

- `GET` : [`/to-do/home`](http://0.0.0.0:8000/to-do/home) show the to-do in the ordre of creation with some action 
- `GET` : [`/to-do/create`](http://0.0.0.0:8000/to-do/create) display form to create a to-do
- `POST` : `/to-do/create` allow to create in the database the to-do
- `DELETE` : `/to-do/owner/<id>` delete all the to-do of the account in <id>
- `DELETE` : `/to-do/owner/done/<id>` delete all the to-do done of the account in <id>
- `DELETE` : `/to-do/delete/<id>` delete the to-do with the <id>
- `GET` : `/to-do/edit/<id>` show the forum to edit a to-do
- `PUT` : `/to-do/edit/<id>` put to save the new data
- `PUT` : `/to-do/set_progress/<id>/<value>` allow to just change the progress value