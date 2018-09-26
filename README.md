# fq

Quick and dirty client for YSHI file upload service

## Build

```
cargo build --release
```

### Prerequisites

#### Linux/WSL

```
sudo apt-get install xorg-dev
```

## Use

```
fq --help
fq upload cat.jpg
```

fq will copy the URL to your clipboard.

### Windows integration

#### Add

1. Download fq somewhere, and create a config file (see [test_config.toml](test/fixtures/test_config.toml))
2. Modify `add.reg` to add a context menu in the shell
3. Run the modified `add.reg`

#### Remove

1. Run `win/remove.reg`.
