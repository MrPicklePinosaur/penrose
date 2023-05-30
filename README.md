
<div align="center">

# penrose

pinosaur's wm built using [penrose](https://github.com/sminez/penrose).

</div>

## INSTALLATION

first build `penrose`:
```
$ just build
```

next add the following to your `.xinitrc` or equivalent:
```
[PATH_TO_REPO]/target/release/penrose &> ~/.penrose.log
```

## SETUP FOR DEVELOPEMENT

ensure that you have the rust nightly toolchain (nightly clippy and rustfmt features ares used):
```
$ rustup install nightly
```

next install dev git hooks:
```
$ just devsetup
```

## TODO

- Load options from ron config file
- Status bar (lemon bar?)
- Select a specific layout
- 3 column layout

