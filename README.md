# caseta_codec

## What is it?

This is a little project to listen to messages coming across a [Lutron Caseta Smart Hub PRO](https://www.casetawireless.com/us/en/pro-products). Typically, someone will use a Lutron Caseta system to integrate parts of their home's electrical setup into a smart home (like Apple Home, Google Assistant, and others). The Lutron Caseta switches and Pico remotes fit standard decora-style electrical wall plates, so they don't stand out when installed next to existing switches, and outlets.

What does stand out, however, is the telnet interface that the `Smart Hub PRO` exposes (which the standard hub, sadly, does not). If you're connected, you can press a button on a pico remote and see a message of that button press come through:

```shell
nc yourcasetahost.run 23
login # type in your login and press enter
password # type in your password and press enter
GNET> ~DEVICE,2,3,3 # et voila! button press and release events will start showing up
~DEVICE,2,3,4
~DEVICE,2,3,3
~DEVICE,2,3,4
#...
```

## Why is this cool?

If you can listen to pico remote button events, you can write some software to _respond_ to those button events. Maybe they can turn on smart lights or trigger other home automation?

## How do I run this project?

Clone the repo and run `cargo build` from the project root. Then `cargo run` the project with these environment variables

```shell
CASETA_HOST=...
CASETA_PORT=...
CASETA_USERNAME=...
CASETA_PASSWORD=...
```

you should now see the messages we saw with `nc` earlier coming out of the rust project's logs
