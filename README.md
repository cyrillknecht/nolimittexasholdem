# No Limit Texas Hold'Em Poker Game by Pokerers

![Poker-logo](./assets/poker_logo.png?raw=true)

This is an implementation of the game "No Limit Texas Hold'Em Poker" developed by team Pokerers as a course project for the Software Engineering Course at ETH Zurich. You can read the game's rules [here](https://upswingpoker.com/poker-rules/).

## Installation
### Prerequisites
- Rust must be installed on your system. If it is not installed, you can download and install it from
  the [official Rust website](https://www.rust-lang.org/tools/install).
- Git must be installed on your system. If it is not installed, you can download and install it from
  the [official Git website](https://git-scm.com/downloads)

First of all, clone all submodules of this repository.

````
git clone --recurse-submodules https://gitlab.ethz.ch/thherzog/nolimittexasholdem.git 
````

### Server Installation Instructions

To build the server, execute the following command in the `nolimittexasholdem-server` directory:

```
cargo build --release
```

### Client Installation Instructions
*The instructions for installing the client are nearly the same as for the original Lama Project provided by the course.*

#### 1. Compile instructions
This project only works on UNIX systems (Linux / MacOS). We recommend using [Ubuntu](https://ubuntu.com/#download), as it offers the easiest way to set up wxWidgets. Therefore, we explain installation only for Ubuntu systems. The following was tested on an Ubuntu 20.4 system, but should also work for earlier versions of Ubuntu.

**Note:** If you create a virtual machine, we recommend to give the virtual machine **at least 12GB** of (dynamic) hard drive space (CLion and wxWidgets need quite a lot of space).

##### 1.1 Prepare OS Environment

###### Ubuntu 20.4
Execute the following commands in a console:
1. `sudo apt-get update`
2. `sudo apt-get install build-essential` followed by `sudo reboot`
3. if on virtual machine : install guest-additions (https://askubuntu.com/questions/22743/how-do-i-install-guest-additions-in-a-virtualbox-vm) and then `sudo reboot`
4. `sudo snap install clion --classic` this installs the latest stable CLion version
5. `sudo apt-get install libwxgtk3.0-gtk3-dev` this installs wxWidgets (GUI library used in this project)


##### 1.2 Compile Code
1. Open Clion
2. Click `File > Open...` and there select the **/sockpp** folder of this project
3. Click `Build > Build all in 'Debug'`
4. Wait until sockpp is compiled (from now on you never have to touch sockpp again ;))
5. Click `File > Open...` select the **/nolimittexasholdem-client** folder
6. Click `Build > Build all in 'Debug'`
7. Wait until Poker-server, Poker-client and Poker-tests are compiled


## Run The Game
### 1. Run the Game with the setup script

For the run_game script to work, the `nolimittexasholdem-client` directory must be in the same directory as the `nolimittexasholdem-server` directory.
The run_game script was only tested on macOS. It might not work on other systems. In this case please follow the manual instructions below to run the game.

To run the game after following the installation instructions for building the server and client, you need to do the following:

1. Open a console in the `nolimittexasholdem-client` directory

2. Run the following command to start the game:
````
bash run_game.sh <number_of_players> <role>
````

**number_of_players**: The number of clients you want to generate on your machine. Default is 1.

**role**: If you specify role *host*, the game server will be started on your machine.
Else just the specified number of clients will be started.

Both parameters are optional.

Example with one client and the server running on different machines:
````
bash run_game.sh
````

Example with one client and the server running on the same machine:
````
bash run_game.sh 1 host
````

### 2. Run the Game manually without the setup script
If the setup script does not work for you, you can also run the game manually.

1. Open a console in the `nolimittexasholdem-server` directory
2. Run the following command to start the server:
    ````
    cargo run --release
    ````
3. Open a console in the `nolimittexasholdem-client/cmake-build-debug` directory
4. Run the following command to start the client:
    ````
    ./Pokerers-client
    ````

## Run The Tests

To run the tests, execute the following command in the `nolimittexasholdem-server` directory:

```
cargo test
```

The system will run all tests and print the results to the console.
The output will only show three tests, but these are actually 20 tests in total which are bundled together.

## Generate The Documentation

To generate documentation for the project, execute the following command in the `nolimittexasholdem-server` directory:

```
cargo doc --no-deps --open
```

This command generates HTML documentation for the project and opens it in your default browser.

