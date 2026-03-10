# Recho

A very simple program that listens on port 4000 and echoes to the terminal the
details of any HTTP requests sent to it, including method, path, query params,
headers and request body.

The output uses log level INFO, so you need to set an environment variable
accordingly:

    RUST_LOG=info recho

## TODO

Allow listen port to be configurable.

Allow request part colours to be configurable.
