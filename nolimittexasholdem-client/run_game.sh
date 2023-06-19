#!/bin/bash

# Kill any existing Lama-server and Lama-client processes
killall cargo
killall Pokerers-client

cd cmake-build-debug || exit

# Set the default number of Lama-client instances to 1 if not provided
num_clients=${1:-1}

# Run Lama-client n times
for i in $(seq 1 "$num_clients"); do
    open -a Terminal cd cmake-build-debug && exit; ./Pokerers-client &
done

# Check if the second argument is provided and equals "host"
if [[ "$2" == "host" ]]; then
    # Run Poker-Server
    cd ../../nolimittexasholdem-server || exit
    cargo run --release
fi



