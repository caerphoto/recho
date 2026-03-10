# Recho

A very simple program that listens on port 4000 and echoes to the terminal the
details of any HTTP requests sent to it, including method, path, query params,
headers and request body.

The output uses log level INFO, so you need to set an environment variable
accordingly when running the program:

    RUST_LOG=info recho

If the program is able to log output successfully, it will respond with a HEAD
200.

## Example

    curl http://localhost:4000/testing?foo=bar&baz=qux --data "hello this is curl!" --request POST

Log output:

```
[2026-03-10T11:13:20Z INFO  recho] Request path: POST /testing
    Query:
    {
        "baz": "qux",
        "foo": "bar",
    }
    Headers:
    {
        "host": "localhost:4000",
        "user-agent": "curl/8.7.1",
        "accept": "*/*",
        "content-length": "19",
        "content-type": "application/x-www-form-urlencoded",
    }
    Body:
    hello this is curl!
```

## TODO

Allow listen port to be configurable.

Allow request part colours to be configurable.
