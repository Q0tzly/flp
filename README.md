# flp
Get fullpath of file or working directory

## How to Install
## install with cargo install
```
  cargo install flp 
```
## uninstall
```
  cargo uninstall flp
```

## install to ~/.cargo/bin
``` 
  git clone https://github.com/Q0tzly/flp.git
  cd flp
  cargo install --path .
```
## uninstall
```
  cd flp
  cargo uninstall

  or

  rm  ~/.cargo/bin/flp
```

## install to /usr/local/bin
```
  git clone https://github.com/Q0tzly/flp.git
  cd flp
  cargo build --release
  cp target/release/flp /usr/local/bin
  cd .. && rm -rf flp
```

## uninstall
```
  rm /usr/local/bin/flp
```

## Usage
```
  flp -h, --help     put help
  flp -v, --version  put version
  flp                put working dir
  flp <PATH>         put full path of file or dir
```

## Copyright
Copyright (c) 2024: [Q0tzly](https://github.com/Q0tzly)

See the [LICENSE](https://github.com/Q0tzly/flp/blob/main/LICENSE.txt)
