# Simple TOTP

## why

Many services started requiring 2FA recent time. Marketing immediately responded to the demand
offering nice and powerful solutions. But usual thing is the solutions are too complex.

If you ask AI to implement TOTP then it will offer 100 line code for that. Obviously,
I decided to wrap AI response in a working app without too much extra dependencies AI so likes.

## how
It's CL| app with the web interface. If you run it in a terminal, you operate using regular command
operands, otherwise you can get response in JSON and display it in a browser.

## CLI & Web interface
The program expects arguments as an HTTP query string specified in env value `QUERY_STRING`. When the program
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
Obviously that CGI Rust app can be in the same directory, where the rest of web resources. It should be
reflecting in *mapping* though.

The program needs to know *HOME* directory to successfully function. It gets obtained automatically at
the first run when the program invoked from a terminal. It doesn't matter how the program executed in the case.

If _.home_ file wasn't created for some reason. You can create it manually with a string with full HOME directory path.
The file has to be in the same directory as *simtotp* executable;

## packaging
There is the _package_ **RustBee** script goal to convenient package the application. You sill may need to edit _env.conf_
after unzipping the package to avoid port conflict.

## References
1. [hmac description](https://en.wikipedia.org/wiki/HMAC)
2. [hmac algorithm in JS](https://gist.github.com/stevendesu/2d52f7b5e1f1184af3b667c0b5e054b8)
3. [RFC 6238](https://datatracker.ietf.org/doc/html/rfc6238)