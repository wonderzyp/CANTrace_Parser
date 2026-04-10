# CAN BLF Decoder
A cross-platform CLI tool written in Rust for decoding CAN bus .blf (Binary Logging Format) files into human-readable text.


## Installation
Ensure you have the Rust toolchain installed, then clone the repository and build the release version:

```bash
cargo build --release
```
The binary will be generated in ./target/release/.

## Usage

1. Parse a Single File
```bash
./target/release/can_trace_parser path/to/log.blf
```

2. Batch Parse a Directory
```bash
./target/release/can_trace_parser dir_contains_blfs
```

## Sample Output
```bash
 ~/can_trace_parser ./target/release/can_trace_parser ./blf
Processing "Can-20260310090708274-20260310091009274.blf"
Processing "Can-20260310090403925-20260310090407199.blf"
```

Result in resu dir:
```text
// resu/20260310-170403---20260310-170406.txt

[2026-03-10 17:04:03.706]	chn=1	 0xAA 	 dlc=8	[C1, 01, 11, 13, D2, 00, 00, 00]
[2026-03-10 17:04:03.706]	chn=2	 0x2DD 	 dlc=8	[A3, D4, D4, D0, 00, 00, 00, 00]
[2026-03-10 17:04:03.706]	chn=2	 0x111 	 dlc=8	[AA, B4, 00, 00, 00, 00, 00, 00]
```

