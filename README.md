# Simple TOTP

## why

Many services started requiring 2FA recent time. Marketing immediately responded to the demand
offering nice and powerful solutions. But usual thing is the solutions are too complex.

If you ask AI to implement TOTP then it will offer 100 line code for that. Obviously,
I decided to wrap AI response in a working app without too much extra dependencies AI so likes.

## how
It's CLI app with the web interface. If you run it in a terminal, you operate using regular command
operands, otherwise you can get response in JSON and display it in a browser.

## CLI & Web interface
The program expects arguments as an HTTP query string specified in the env value `QUERY_STRING`. When the program
runs as CLI app, it detects that no such env variable, and then generates it based on the program arguments.

## configuring the [Simple HTTP](https://github.com/vernisaz/simhttp)
The following fragment has to be added in the mapping section of the server:
```
    {"path":"/totp/bin", "_comment_": "Simple TOTP using Rust",
   "CGI": true,
   "translated": "./../simtotp"},
   {"path":"/totp",
   "translated": "./../simtotp/html"}
```
Obviously that CGI Rust app can be in the same directory, where the rest of web resources reside. It should be
reflecting in *mapping* though.

The program needs to know *common config* directory to successfully function. It gets obtained automatically at
the first run when the program invoked from a **terminal**. It's okay if the program reported some errors.

If _.config_ file wasn't created for some reason. You can create it manually with a string 
containing a fully qualified _common config_ directory path. 
The file has to be in the same directory as *simtotp* executable;

## building
In case if no Rust executable for your platform or you like to build everything by yourself, you will need to:

1. obtain [rb](https://github.com/vernisaz/rust_bee/releases/tag/v1.15.01) or build from [source](https://github.com/vernisaz/rust_bee)
2. clone [base32](https://github.com/andreasots/base32/tree/master), [SimTime](https://github.com/vernisaz/simtime),
[simweb](https://github.com/vernisaz/simweb),
[simjson](https://github.com/vernisaz/simjson), [SimConfig](https://github.com/vernisaz/simconfig),
and [simscript](https://github.com/vernisaz/simscript) repositories

First, build all dependencies by executing _rb_ in their repositories. _simscript_ doesn't need to be built. [bee.7b](https://github.com/vernisaz/simtotp/blob/master/dep%20crates/README.md)
is provided for _base32_. And after building the dependencies, execute the _rb_ here to build a final application.

## packaging
There is the _package_ **RustBee** script goal to convenient package the application. You sill may need to edit the _env.conf_
after unzipping the package to avoid a port conflict.

## accessing
An access URL looks like: `http://localhost:3000/totp/`, the ending slash is essential.

## references
1. [hmac description](https://en.wikipedia.org/wiki/HMAC)
2. [hmac algorithm in JS](https://gist.github.com/stevendesu/2d52f7b5e1f1184af3b667c0b5e054b8)
3. [RFC 6238](https://datatracker.ietf.org/doc/html/rfc6238)