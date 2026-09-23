# jwtdecode

A standalone CLI to instantly decode and inspect JSON Web Tokens (JWTs). Fills the gap left by web-based decoders (like jwt.io) by providing a fast, secure, native way to inspect headers and claims right in the terminal.

## Status

**built, untested**: the JWT parsing and base64 decoding logic is built. It has not been heavily tested with varied padding or encoding configurations in this sandbox environment.

## Installation

```sh
cargo install --path .
```

## Usage

```sh
jwtdecode eyJhbG...

# Or via stdin
echo "eyJhbG..." | jwtdecode
```
