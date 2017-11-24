[![Build Status](https://travis-ci.org/mexus/fritzbox-logs-analyzer.svg?branch=master)](https://travis-ci.org/mexus/fritzbox-logs-analyzer)

# Fritz!box logs analyzer

A set of tools that aim to parse and analyze logs from a Fritz!Box routers.

Currently there is just a single application developed that combines logs from
files into a structured (and compressed) *database* which can be reused for some
real analysis, which is only yet to be done.

## Obtaining logs

To load the current logs from your fritz!box router you can use a simple python
module like [fritzconnection](https://pypi.python.org/pypi/fritzconnection).
Here's a one-liner for it (on linux):

```
% python -c "from fritzconnection import FritzConnection; \
             from getpass import getpass; \
             conn = FritzConnection(password=getpass()); \
             logs = conn.call_action('DeviceInfo:1', 'GetDeviceLog'); \
             print(logs['NewDeviceLog'])" > logs.txt
```

It will ask you for your password (i.e. the one you enter to access the router
via web browser) and save all available logs to the 'logs.txt' file.

## Parsing the logs

To append a log to a database (or to create a new database) run the following command:

```
$ fritzbox_logs_analyzer --db-path ~/fritz-box-logs/combined.db logs.db \
                         --logs ~/fritz-box-logs/2017-11-13_20.57.txt \
                         --compression-level 9
```

Or with `cargo run` if you are working with the sources:

```
$ cargo run -- --db-path ~/fritz-box-logs/combined.db logs.db \
               --logs ~/fritz-box-logs/2017-11-13_20.57.txt \
               --compression-level 9
```

## Plans

- [ ] Parse multiple textual logs files simultaneously.
- [ ] Consider to fetch the logs directly from a router (avoid running a python
      script).
- [ ] Plot disconnections statistics.
- [ ] Plot DSL bandwidth statistics.
