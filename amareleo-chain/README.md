# Amareleo-Chain - disposable, developer friendly, Aleo chain instances.

## What is Amareleo-Chain?

* A process manager that launches multiple snarkOS instances and cleans up the ledger storage on exit.

* Relieves developers from the details of running multiple snarkOS instances to have a functioning test environment.


<BR />

## Install

Amareleo-chain was tested against the latest snarkOS release to date (v2.2.7) on __Ubuntu 22.04 (LTS)__.


* [Check the minimum requirements for running snarkOS](https://github.com/AleoNet/snarkOS?tab=readme-ov-file#2-build-guide).

* Install the latest snarkOS
    ```BASH
    git clone https://github.com/AleoNet/snarkOS.git    
    cd snarkOS
    git checkout -B  mainnet

    ./build_ubuntu.sh
    cargo install --locked --path .
    ```

* Install amareleo-chain
    ```BASH
    git clone git@github.com:kaxxa123/amareleo.git
    cd amareleo/amareleo-chain
    cargo install --path .
    ```

<BR />


## Run

If everything is correctly installed launch an Aleo developer chain:

```BASH
amareleo-chain
```

![Amareleo-Chain](./docs/amareleo-chain.png)

Wait until all four nodes are started. Next you can peak into the logs of each node by entering the node number (0 to 3) or simply let it run and go test your leo programs. Afterall that is the main purpose of amareleo-chain.

Once ready hit q to terminate amareleo-chain.

<BR />

## Configuration

Amareleo-chain is configurable using a json configuration file.

amareleo-chain will automatically create the default configuration under: <BR />
~/.amareleo/chain-cfg.json

One may then directly edit this file to customize the snarkOS startup.

Alternatively, one can create a fresh copy of the configuration file and pass it as a
command-line parameter to amareleo-chain, overriding the default configuration path.

<BR />

## Command-line Options

For a full list of command-line options run:

```BASH
amareleo-chain --help
```

* __init__ - `amareleo-chain init` will generate a copy of the default configuration json file.


* __run__ - `amareleo-chain run` allows starting the chain with a custom configuration
    and a custom chain storage folder. However, when using a custom chain storage folder
    amareleo-chain won't delete the chain on exit. Thus, the chain state is retained across runs.


Running `amareleo-chain` is equivalent to running `amareleo-chain run` without passing any
additional arguments.



<BR />

## What Next?

There is more we would like to add. We want the tool to start showing critical information like the processing of transactions and the mining of new blocks whilst filtering out other information. Stay tuned and post an issue if you would like to add more functionality.
