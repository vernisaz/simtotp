# Simple TOTP

## why

Many services started requiring 2FA recent time. Marketing immediately responded to the demand
offering nice and powerful solutions. But usual thing is the solutions are too complex.

If you ask AI to implement TOTP then it will offer 100 line code for that. Obviously,
I decided to wrap AI response in a working app without too much extra dependencies AI so likes.

## how
It's CLI app with the web interface. If you run it in a terminal, you operate using regular command
operands, however you will get a response in JSON format which is a browser friendly.

## CLI & Web interface
The program expects arguments as an HTTP query string specified in the env value `QUERY_STRING`. When the program
runs as a CLI app, it detects that no such env variable, and then generates it based on the program arguments.

## configuring the [Simple HTTP](https://github.com/vernisaz/simhttp)
The following fragment has to be added in the mapping section of the server _env.conf_
```
    {"path":"/totp/bin", "_comment_": "Simple TOTP using Rust",
   "CGI": true,
   "translated": "./../simtotp"},
   {"path":"/totp",
   "translated": "./../simtotp/html"}
```
Obviously that CGI Rust app can be in the same directory, where the rest of web resources reside. It should be
reflecting in *mapping* though.

The program needs to know *common config* directory to successfully work. It gets obtained automatically at
the first run when the program invoked from a **terminal**. It's okay if the program reported some errors.

If _.config_ file wasn't created for some reason. You can create it manually with a string 
containing a fully qualified _common config_ directory path. 
The file has to be in the same directory as *simtotp* executable. CGI app can't obtain the directory
from the environment, because it runs sandboxed.

Any other web server capable to run CGI scripts can be also used.

## building
In case if no Rust executable for your platform or you like to build everything by yourself, you will need to:

1. obtain [rb](https://github.com/vernisaz/rust_bee/releases/tag/v1.15.06) or build it from [source](https://github.com/vernisaz/rust_bee)
2. clone [base32](https://github.com/andreasots/base32/tree/master), [SimTime](https://github.com/vernisaz/simtime),
[simweb](https://github.com/vernisaz/simweb),
[simjson](https://github.com/vernisaz/simjson), [SimConfig](https://github.com/vernisaz/simconfig),
and [simscript](https://github.com/vernisaz/simscript) repositories

First, build all dependencies by executing _rb_ in their repositories. _simscript_ doesn't need to be built.
[bee.7b](https://github.com/vernisaz/simtotp/blob/master/dep%20crates/README.md)
is provided for _base32_. And after building the dependencies, execute the _rb_ here to build the final application.

You can generate a deplyment package after by executing `rb package`.

## packaging
There is the **RustBee** script target – `package` for a convenient packaging of the application.
You sill may need to edit the _env.conf_
after unzipping the package to avoid a port conflict.

## accessing
An access URL looks like: `http://localhost:3000/totp/`, the ending slash is essential.

## usage
The password is used for encryption of the stored data. Select any, and then use it when work with the application. 

The secret is stored under **namespace/account**. Select the desired **namespace/account** when you 
need to generate a code after.

If you lost or forgot the password, then execute the `uninstall` script and then fill in the
application data again. It's recommended to create a backup copy of the data and store 
on a flash drive or other secured backup storage with easy to remember or no password and then, 
use it in a case of an emergency.

The application has benefits against PWA as [pwa-otp](https://github.com/maxerenberg/pwa-otp), because
it can be shared between several devices. It makes it an ideal for a private cloud.


## uninstall
The installation package contains `uninstall` script. It will delete the application data, and then 
the application directory can be safely removed using a file manager, or a command line tool.

## security
There is no security risks besides of a brutal force attack, since the app sends password with every request.
A possibility of such attack can be reduced my introducing a mandatory throttling of requests, or even completely
block them from certain IPs reached max possible number of unsuccessful requests.

## references
1. [hmac description](https://en.wikipedia.org/wiki/HMAC)
2. [hmac algorithm in JS](https://gist.github.com/stevendesu/2d52f7b5e1f1184af3b667c0b5e054b8)
3. [RFC 6238](https://datatracker.ietf.org/doc/html/rfc6238)