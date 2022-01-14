#!/bin/bash
source .env

echo "You can change the mode by changing the value of LAUNCH_MODE in the .env file"
echo "'r' -> release" 
echo "'d' -> debug"
echo "'t' -> test"

diesel migration run

case $LAUNCH_MODE in

  "r")  echo "Mode is: release 'r'"
        cargo run --release src/main.rs
        exit 0;;

  "d")  echo "Mode is: debug 'd'"
        cargo run src/main.rs
        exit 0;;

  "t")  echo "Mode is: test 't'"
        cargo test
        exit 0;;

  *) echo "LAUNCH_MODE = '$LAUNCH_MODE' incorrect , available option are (r, d, t)";;
esac
exit 1;
