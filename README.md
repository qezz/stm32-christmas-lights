# Stupid christmas tree lights with stm32f103c8t6

![tree with lights](https://i.imgur.com/DAaX9BT.jpg)

Currently it runs the red, blue, and green colors through the led
strip. Feel free to tweak anything you don't like!

## Hardware requirements

* stlink v2
* stm32f103c8t6 or anything you want
* addressable rgb led strip
  * data pin to PB15
  * 5v to 5v
  * ground to ground

## Software requirements

* cargo and rustc (tested with rustc 1.51-nightly)
* rust target `thumbv7m-none-eabi
* stlink utils

### on nixos

Feel free to use `default.nix` environment with the needed
requirements. Call `nix-shell` within a working directory, or take a
look at `direnv` plugin for you shell

```
$ nix-shell
```

## Build and flash


```
make build
make flash
```

Flashing requires superuser rights, so `sudo` is used. (Maybe it is
better to let the user decide to use `sudo`, so it will be `sudo make
flash`... Let me know!) On macOS this could be excessive. In case of
success, you'll see the following in the output:

```
...
2020-12-30T20:06:29 INFO flash_loader.c: Successfully loaded flash loader in sram
 10/10 pages written
2020-12-30T20:06:30 INFO common.c: Starting verification of write complete
2020-12-30T20:06:30 INFO common.c: Flash written and verified! jolly good!
```

## License

MIT or Apache 2.0

## Author

Sergey Mishin
