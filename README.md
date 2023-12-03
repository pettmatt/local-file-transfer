# About

In its core "Local file transfer application" or LFT is a REST server that allows users to move files between devices that are in the same network using a browser. User can create their own web interface and modify the server itself, but in order to use the server they need to host it somewhere, locally hosted is the recomended way to go. You can either donwload the binaries or build the server and the interface yourself. More detailed instructions can be found in `file-transfer-server` and `web-interface` directories.

## Good to know

There are multiple things that could be improved, for example PWA-application as the interface could improve user experience and the server could be more secure, which could enable it to be used in greater scale. At the moment this application is recommended to be used in smaller scale where couple of people share their files between reasonable amount of devices.

# Intalling and running

## The server

Server is build using Rust, so running the application needs Rust in order to run on the machine ([how to install](https://www.rust-lang.org/tools/install)).

Environment variables:
```
HOST_PORT=3003
HOST_ADDRESS=localhost
THREAD_POOL_COUNT=1
```

Compile and run the program:
```cmd
cargo run
```

## Web interface

Interface is build on React library and requires Node.js and npm to be installed on the machine. 

```cmd
# Install depedencies
npm intall

# Run the application
npm run start
```

## Docker

The application can be also run in docker, which runs the application in containers and doesn't require other than Docker to run.

### Web interface

**NOTE: Use network address to visit the application.**

```cmd
docker build -t web-interface .
docker run -p 5173 web-interface
```

### Rust
```cmd
docker build -t file-server .
docker run -e HOST_ADDRESS=0.0.0.0 -e THREAD_POOL_COUNT=1 -p 7878:7878 file-server
```