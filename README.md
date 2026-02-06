# Ajazz AK820 Pro Time Sync (Linux)

The official Ajazz driver is Windows-only. This tool reverse-engineers the USB HID protocol to allow Linux users to keep their keyboard's screen clock accurate.

## Installation
1. Build the binary:
```
cargo build --release
```
2. Run it:
```
sudo ./target/release/ak820pro-time-sync
``` 

## Disclaimer

This software is provided "as is", without warranty of any kind. While this tool has been tested on the Ajazz AK820 Pro, I am not responsible for any issues that may arise from its use.

## License
MIT Licence
