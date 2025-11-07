## iroh-cli

Minimal code to test an iroh connection.

Much better code examples:

https://github.com/n0-computer/iroh-doctor/blob/main/src/commands/accept.rs
https://github.com/n0-computer/iroh-doctor/blob/main/src/commands/connect.rs
https://github.com/n0-computer/iroh/blob/dd99737c12c553ece2607e5e74d605751a637397/iroh/src/endpoint.rs#L2550

Much better testing tool:

https://github.com/n0-computer/iroh-doctor/

## Usage

If you're running a test from two computers, you can use the default keys.

Computer 1:

> cargo endpoint create
> cargo endpoint read

(get the endpoint id from computer 1 to computer 2 however is most convenient)

> cargo endpoint listen

Computer 2:

> cargo endpoint create
> cargo endpoint connect {computer1_endpoint_id}

Your devices are now free to move about the internet

## References

https://iroh.computer/
https://github.com/n0-computer
