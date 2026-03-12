# IFOCOB
#### and more

## DISCLAIMER

HEAVILY IN DEVELOPMENT

VERSION: 0.1

## FAQ

### What is this?

This is an open source project to create a bridge from rare image formats to your image viewer! (and conversions)

## Why is this?

I am sad and disappointed with the rate of adaptation for formats that are so good, for this reason I created this project to make a centralized opensource application to easily open and transform rare formats

## Installing

### Linux

#### Dependencies

* just
* cargo
* rustc

#### Commands

```bash
git clone https://github.com/quotequack/ifocob
cd ifocob
just install
```

### Nixos

Add "github:quotequack/ifocob" to your flake
Add ifocob.packages.x86_64-linux.default

### Window/Macos
 
To be determined

## Formats supported

* png
* jpeg
* bmp
* qoi

## Contributing

Instructions for adding codecs in contributing.md