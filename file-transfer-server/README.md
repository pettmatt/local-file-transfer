# Instructions

## Starting the server

You can start the server by executing the "file-transfer-server.exe" file after compiling the binaries. Note that the application can be blocked by firewalls, you might need to expose the port manually to your local network and the final host address depends on the host machine. Final note, the exe file just opens a terminal and shows the address that the server is hosted on, by closing this terminal you close the server.

If you're not a tinkerer you can ignore the instructions below.

## Env file

If you want to tinker you need to download the source code. You can change some values of the server by changing values in the `.env` file. The file should be put in the root of the application (in the same directory as this `README.md` file). Through the file you can change the thread pool, the host address and the port.

Here's the default structure of the `.env` file with default values.

```
THREAD_POOL_COUNT=2
HOST_ADDRESS=127.0.0.1
HOST_PORT=7878
```

If you want to change the values you also need to build the application with `cargo build`, but if you haven't worked with Rust-language you need to install the language on your own machine in order to build the application.

## API routes

The application is a simple REST API that manages files locally. Note that users can identify themselves through username, which is used to separate the files by putting them in a new directory inside the `uploads` directory. This doesn't really do anything, but if you want to manage the files manually you can easily navigate the `uploads` directory if there are multiple people or users identified. If the username is not added in the web interface the files will be stored in the root of `uploads` directory. Below you can see what routes the application uses:

```
GET /files
Returns list of files.

DELETE /files
Deletes a single file, requires "file_name" and "username" query strings.

GET /download
Downloads a file utilizing a stream, requires "file_name" and "username" query strings.
*Username can be empty

POST /send
Uploads a file, requires a payload in the body and "username" query string.
*Username can be empty
```

Below you can see the structure of the file object that the application returns.

```rs
{
    "name": string,
    "size": number,
    "type": string,
    "owner": string
}
```
