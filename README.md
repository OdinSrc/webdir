# webdir

`webdir` is a simple Rust program that serves directory files in a HTTP web server. It's designed to be similar to Python's built-in `http.server` module.

## Features

- Serves files from a specified directory
- Customizable port number and bind address

## Installation

You can install `webdir` using Cargo, the Rust package manager:

```bash
cargo install webdir
```


## Usage

To start the server, run the following command:

```bash
webdir
```


By default, the server will listen on port `8000` and bind to all available interfaces.

You can specify a custom port number using the `-p` or `--port` option:

```bash
 webdir -p 8080
 ```
 
 
You can specify a custom bind address using the `-b` or `--bind` option:

```bash
webdir --bind 127.0.0.1
```


You can specify a custom directory to serve using the `-d` or `--dir` option:

```bash
 webdir --dir /path/to/directory
 ```
 
 ## Why webdir?


There are other similar crates in Rust for serving directories over HTTP, but I found them to be too complex and unnecessary for simple directory serving purposes. I created `webdir` for my personal use as a lightweight and easy-to-use alternative to those crates. Feel free to use it if you find it useful.


 
## Contributing

If you find any issues or have any suggestions for improvements, feel free to open an issue or submit a pull request.
