# fleece

A tool for parsing Good-Feel's BSON files.

## Usage

### Decode

To convert a BSON file to JSON, use the `decode` command.

```
fleece decode -i my_file.bson -o output.json
```

You can also use the `-p` or `--pretty` flags to make your JSON output pretty.

```
fleece decode -i my_file.bson -o output.json -p
```

```
fleece decode -i my_file.bson -o output.json --pretty
```

### Encode

To convert a JSON file to BSON, use the `encode` command.

```
fleece encode -i my_file.json -o output.bson
```

By default, files are encoded in big endian. You can specify endianness using the `-e` or `--endian` flags (`big` or `little`).

```
fleece encode -i my_file.json -o output.bson -e little
```

```
fleece encode -i my_file.json -o output.bson --endian little
```

## Advanced Options

### Endianness

When decoding, the tool defaults to `auto` to detect the endianness based on the BSON header. You can manually override this if necessary:

```
fleece decode -i my_file.bson -o output.json --endian big
```

#### BSON Version

When encoding, the default BSON version is `3`. You can specify a different version with the `-v` or `--version` flags.

```
fleece encode -i my_file.json -o output.bson --bson-version 3
```
