# SIM800 Reader

Simple utility to receive SMS messages using SIM800 module serial port api.

## Build

To build `sim800-reader` from source code use following command:

```sh
cargo build --release
```

## Command line options

* `-h` (long `--help`) - print short help;
* `-s` (long `--serial-port`) <SERIAL_PORT> - select serial port to connect;
* `-b` (long `--baud-rate`) <BAUD_RATE> - set serial port baud rate, default: 115200;
* `-l` (long `--list-messages`) - list SMS messages;
* `-d` (long `--delete-messages`) - Clean SMS messages;

## License
[license]: #license

Source code is primarily distributed under the terms of the MIT license. See LICENSE for details.
