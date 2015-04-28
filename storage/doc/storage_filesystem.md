
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

```
version: positive fixint
#flags (not yet implemented, feature flags)
value: bin 8; bin 16; bin 32 (using the smallest possible)
key: bin 8; bin 16; bin 32 (using the smallest possible)
```
