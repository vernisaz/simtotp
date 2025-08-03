# Simple TOTP

## why

Many services started requiring 2FA recent time. Marketing imediately responded to the demand
offering nice and powerful solutions. But usual thing is the solutions are too complex.

If you ask AI to implement TOTP then it will offer 100 line code for that. Obviously,
I decided to wrap AI response in a working app without too much extra dependencies AI so likes.

## how
It's CL| app with the web interface. If you run it in a terminal, you operate using a regular command
operands, otherwise you can get response in JSON and display in a browser.

## CLI & Web interface
The program expects arguments as an HTTP query string specified in env value `QUERY_STRING`. When the program
runs as CLI app, it detects that no such env variable, and then generates it based on the program arguments.

## References
1. [hmac description](https://en.wikipedia.org/wiki/HMAC)
2. [hmac algorithm in JS](https://gist.github.com/stevendesu/2d52f7b5e1f1184af3b667c0b5e054b8)
3. [RFC 6238](https://datatracker.ietf.org/doc/html/rfc6238)