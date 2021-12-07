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
            Length of device chains [default: 1]

    -h, --help
            Print help information

    -o, --output <OUTPUT>
            Output file [default: abusim.yml]
```
