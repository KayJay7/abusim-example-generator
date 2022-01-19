# abusim-example-generator

Example generator for abusim.

A program to testing configurations for [abusim](https://github.com/abu-lang/abusim) of the size defined in the arguments.

## Usage
```
USAGE:
    aeg [OPTIONS]

OPTIONS:
    -a, --devices-number <DEVICES_NUMBER>
            Number of devices [default: 1]

    -b, --chains-number <CHAINS_NUMBER>
            Number of rule chains per devices [default: 1]

    -c, --chain-length <CHAIN_LENGTH>
            Length of rule chains (internal to a device) [default: 1]

    -d, --chain-width <CHAIN_WIDTH>
            Width of (the last level) rule chains [default: 1]

    -e, --devices-width <DEVICES_WIDTH>
            Number of devices activated by a chain [default: 1]

    -f, --devices-length <DEVICES_LENGTH>
            Length of device chains [default: 1]er image [default: abulang/abusim-coordinator:latest]




    -C, --coordinator-image <COORDINATOR_IMAGE>
            Coordinator's dock

        --configuration-version <CONFIGURATION_VERSION>
            Configuration version [default: 1.0]

    -h, --help
            Print help information

    -I, --image <IMAGE>
            Agent's docker image [default: abulang/abusim-goabu-agent:latest]

        --includes <INCLUDES>
            Included files

        --memory-controller <MEMORY_CONTROLLER>
            Memory controller [default: basic]

        --namespace <NAMESPACE>
            Namespace [default: abusim-example]

    -o, --output <OUTPUT>
            Output file [default: abusim.yml]

    -T, --tick <TICK>
            Ticking time [default: 1s]

    -V, --version
            Print version information
```

If compiled in debug mode, `aeg` will also print to stdout the defined options and the entire output YAML as debug information.

## Currently implemented

* [x] devices_number
* [x] chains_number
* [x] chain_length
* [x] chain_width
* [x] devices_length
* [x] devices_width

## Building

The project can be built with cargo. A rust toolchain (stable or nightly) is required for building.

```sh
# git clone to your local disk
git clone https://github.com/KayJay7/abusim-example-generator.git
cd abusim-example-generator

# build debug
cargo build

# build release
cargo build --release

# run with arguments
cargo run -- # your arguments here

# install to your cargo/bin directory
cargo install --path . # from the project root
```
