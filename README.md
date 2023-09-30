
<div align="center">

# PinoWM

pinosaur's wm built using [penrose](https://github.com/sminez/penrose).

</div>

## INSTALLATION

To build an install `PinoWM` with all required config files:
```sh
just install
```

## SETUP FOR DEVELOPEMENT

Ensure that you have the rust nightly toolchain (nightly clippy and rustfmt features ares used):
```sh
rustup install nightly
```

next install dev git hooks:
```sh
just devsetup
```

## TODO

- Load options from ron config file
- Status bar (lemon bar?)
- Select a specific layout

