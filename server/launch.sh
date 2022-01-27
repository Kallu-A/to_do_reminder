#!/bin/bash
source .env

echo "You can change the mode by changing the value of LAUNCH_MODE in the .env file"
echo "'r' -> release" 
echo "'a' -> all (fmt, clippy, test, release)"
echo "'d' -> debug"
echo "'t' -> test"
echo "'c' -> clippy (upgrade code)"
echo "'f' -> fmt (syntax format)"
echo "'rd' -> recreate the database"

# if database doesn't exist launch the creation
if [ ! -f database  ]
then
  # set the json file to the default value
  echo "{\"members\":1,\"to_do\":0,\"connexion\":0}" > data.json

  diesel migration run
  echo "Creation of the database: success"
else
  echo "Link to the database: success"
fi

case $LAUNCH_MODE in

  "r")  echo "Mode is: release 'r'"
        cargo test
        cargo run --release src/main.rs
        exit 0;;

  "d")  echo "Mode is: debug 'd'"
        cargo run src/main.rs
        exit 0;;

  "t")  echo "Mode is: test 't'"
        cargo test
        exit 0;;

        # mode for developers so not display
  "c")  echo "Mode is: clippy 'c'"
        cargo clippy
        exit 0;;

        # mode for developers so not display
  "f")  echo "Mode is: fmt 'f'"
        cargo fmt
        exit 0;;

  "rd") echo "Mode is: redo databse 'rd'"
        # set the json file to the default value
        echo "{\"members\":1,\"to_do\":0,\"connexion\":0}" > data.json
        diesel migration redo
        echo "database redo ... done"
        exit 0;;

  "a")  echo "Mode is: all verif 'a'"
        echo -n "launch fmt: "
        cargo fmt
        echo "finish"
        echo "launch clippy: "
        cargo clippy
        echo "-- finish --"
        echo "launch test: "
        cargo test
        echo "-- finish --"
        echo "launch server: "
        cargo run --release src/main.rs
        exit 0;;

  *) echo "LAUNCH_MODE = '$LAUNCH_MODE' incorrect , available option are (r, d, t)";;
esac
exit 1;
