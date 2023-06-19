# No Limit Texas Hold'Em Poker Client by Pokerers

![Poker-logo](./assets/poker_logo.png?raw=true)

This is a simple C++ client implementation of the game "No Limit Texas Hold'Em Poker" developed by team Pokerers as a course project for the Software Engineering Course at ETH Zurich. You can read the game's rules [here](https://upswingpoker.com/poker-rules/). The rules of of our game are slightly altered, as to facilitate the game's implementation. Following things were altered:
- If there are multiple winners, everyone gets their money reimbursed
- There are eight rounds in total, four to adjust the bets, and four to decide whether one wants to call or fold. In the latter four, players who do not need to call are automatically skipped.
- The raise button sets the TOTAL amount of coins betted. So if you are already betting 150 coins, and want to bet 50, you have to enter 200, not 50.
- The bet should be a multiple of the small blind (which doubles everytime the dealer passes position 0), unless it's an all-in
- An invalid input might be corrected automatically by the server to a valid amount
- Other slight edits were made

This repository only contains the code for the client part of the game.
The server code can be found in this[repository](https://gitlab.ethz.ch/thherzog/nolimittexasholdem-server)

To run the game after following the installation instructions for building the server and client, you need to do the following:

1. Open a console in the `nolimittexasholdem-client` directory

2. Run the following command to start the game:
````console
bash run_game.sh <number_of_players> <role>
````

**number_of_players**: The number of clients you want to generate on your machine. Default is 1.

**role**: If you specify role *host*, the game server will be started on your machine.
Else just the specified number of clients will be started. If want to provide commandline arguments to the server,
start the server manually.

Both parameters are optional.

Example with one client and the server running on different machines:
````console
bash run_game.sh
````

Example with one client and the server running on the same machine:
````console
bash run_game.sh 1 host
````




For the run_game script to work, the `nolimittexasholdem-client` directory must be in the same directory as the `nolimittexasholdem-server` directory.
The run_game script was only tested on macOS. It might not work on other systems. In this case please follow the manual instructions below to run the game.


# Client Installation Instructions
*The instructions for setting up the client are the same as for the original Lama Project provided by the course.*
for installation instructions for the server,
please refer to the server's [README](https://gitlab.ethz.ch/thherzog/nolimittexasholdem-server/-/blob/master/README.md).


## 1. Compile instructions
This project only works on UNIX systems (Linux / MacOS). We recommend using [Ubuntu](https://ubuntu.com/#download), as it offers the easiest way to set up wxWidgets. Therefore, we explain installation only for Ubuntu systems. The following was tested on an Ubuntu 20.4 system, but should also work for earlier versions of Ubuntu.

**Note:** If you create a virtual machine, we recommend to give the virtual machine **at least 12GB** of (dynamic) hard drive space (CLion and wxWidgets need quite a lot of space).

### 1.1 Prepare OS Environment

#### Ubuntu 20.4
The OS should already have git installed. If not, you can use: 
`sudo apt-get install git`

Then use  `git clone` to fetch this repository.

Execute the following commands in a console:
1. `sudo apt-get update`
2. `sudo apt-get install build-essential` followed by `sudo reboot`
3. if on virtual machine : install guest-additions (https://askubuntu.com/questions/22743/how-do-i-install-guest-additions-in-a-virtualbox-vm) and then `sudo reboot`
4. `sudo snap install clion --classic` this installs the latest stable CLion version
5. `sudo apt-get install libwxgtk3.0-gtk3-dev` this installs wxWidgets (GUI library used in this project)


### 1.2 Compile Code
1. Open Clion
2. Click `File > Open...` and there select the **/sockpp** folder of this project
3. Click `Build > Build all in 'Debug'`
4. Wait until sockpp is compiled (from now on you never have to touch sockpp again ;))
5. Click `File > Open...` select the **/cse-lama-example-project** folder
6. Click `Build > Build all in 'Debug'`
7. Wait until Poker-server, Poker-client and Poker-tests are compiled

### 2. Run the Game manually without the setup script
1. Open a console in the `nolimittexasholdem-server` directory
2. Run the following command to start the server:
    ````console
    cargo run --release
    ````
3. Open a console in the `nolimittexasholdem-client/cmake-build-debug` directory
4. Run the following command to start the client:
    ````console
    ./Pokerers-client
    ````
