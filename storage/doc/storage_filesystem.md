
# Information about filesystem storage

DRAFT not yet finished

## Features

- [ ] compression
- [ ] encryption


## Storage on Filesystem

The key value will be hashed with Blake2b 32 byte

git-like objects folder

	Hexadecimal path in the format:
	`xx/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx`

	Example:
	`92/8b20366943e2afd11ebc0eae2e53a93bf177a4fcf35bcc64d503704e65e202`

## File format

Current Version 1 (not yet finished)
Encoded using [msgpack](http://msgpack.org/)
Flat encoding

Format differs based on flags?

Key-Value-Format

```
version: positive fixint (current 1)
flags: uint32 (not yet used, feature flags, key_value object
[optional additional header based on flags]
value: bin 8; bin 16; bin 32 (using the smallest possible)
key: bin 8; bin 16; bin 32 (using the smallest possible)
```


CAS-Format:

```
version: positive fixint (current 1)
flags: uint32 (not yet used, feature flags, cas object)
[optional additional header based on flags]
value: bin 8; bin 16; bin 32 (using the smallest possible)
```

### Flags

Bitflags (u32)

* const FLAG_READONLY 	= 0b00000001,
* const FLAG_DELETEABLE = 0b00000010,
* const FLAG_ENCRYPTED	= 0b00000100, (additional header)
* const FLAG_COMPRESSED	= 0b00001000, (additional header)
* const FLAG_CAS		= 0b00010000,

### Additional Header

For the additional headers there is a special order:

* if flag FLAG_ENCRYPTED is set, the crypto header is the first additional header
* if flag FLAG_COMPRESSED is set, it is the second header (when crypto is set)

Crypto Header:

```
crypo-method: fixstr utf8 (Possible values: "AES", "ChaCha20")
```

Compression Header:

```
compression-method: fixstr utf8 (Possible values: "LZ4")
```


