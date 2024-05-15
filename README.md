# kvs

This is avery simple tool to store key-values. It is backed by a single json
file stored in the user's application [data directory](https://docs.rs/dirs/latest/dirs/fn.data_local_dir.html).

## Usage

Set a key-value pair:
```
$ kvs set <KEY> <VALUE>
```

Print a key's value:
```
$ kvs get <KEY>
```

Remove a key-value pair:
```
$ kvs remove <KEY>
```

List all key-value pairs:
```
$ kvs list 
```

To clear all key-value pairs from the store:
```
$ kvs clear
```
