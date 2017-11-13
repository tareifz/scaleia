# Scaleia
A tiny tool written in Rust to rescale images in a given folder.

---

## Requirements
* [Rust compiler](https://www.rust-lang.org/en-US/install.html)

## How to use
```console
$ cargo run --release <directory> <width> <height>
```

```
<directory>: shouldn't start with '/'
<width>, <height> are % using integers (50 = 50%).
```

## Example

```console
$ cargo run --release myimages/ 50 50
```
This will rescale the images to 50% of the original width and height.

## Known issues
* When the directory path starts with forward or backward slash '/', '\'

## TODO
- Binaries
- GPU implementation