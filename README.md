# kvu

The simplest command line tool to manage key-value pair lines.

```
┌──────────────────────────┐                                         ┌──────────────────────────┐
│ DB_URI=postgres://db/kvu │         ┌─────────────────────┐         │ DB_URI=postgres://db/kvu │
│ DB_USERNAME=username     │──stdin─▶│ kvu DB_USERNAME=kvu │─stdout─▶│ DB_USERNAME=kvu          │
│ DB_PASSWORD=password     │         └─────────────────────┘         │ DB_PASSWORD=password     │
└──────────────────────────┘                                         └──────────────────────────┘
```

## Installation

### From binaries

The [release](https://github.com/jihchi/kvu/releases/latest) page includes precompiled binaries for Linux, macOS and Windows.

### From Cargo

With Rust's package manager [cargo](https://github.com/rust-lang/cargo), you can install kvu via:

```sh
cargo install kvu
```

Note that rust version _1.62.0_ or later is required.

## Usage

> Arguments without any flag works as upsert operation - create new pair if the
key does not exist, or update the value of the key if the key does exist.

```console
$ echo -e "BUCKET=public\nREGION=ap-southeast-1" | kvu TOKEN=348a1912 REGION=eu-north-1
BUCKET=public
REGION=eu-north-1
TOKEN=348a1912
```

### `-c/--create` Create new pair

Does nothing when the key exists.

```console
$ echo -e "BUCKET=public\nREGION=ap-southeast-1" | kvu --create REGION=eu-north-1
BUCKET=public
REGION=ap-southeast-1
```

### `-u/--update` Update existing pair

Does nothing when the key does not exist.

```console
$ echo -e "BUCKET=public\nREGION=ap-southeast-1" | kvu --update TOKEN=348a1912 
BUCKET=public
REGION=ap-southeast-1
```

### `-d/--delete` Delete existing pair

Does nothing when the key does not exist.

```console
$ echo -e "BUCKET=public\nREGION=ap-southeast-1" | kvu --delete REGION
BUCKET=public
```
