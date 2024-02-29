# (Anti) Koch Snowflake generator

This Rust program generates simple (Anti) Koch snowflakes from the commandline in the
[Netpbm format](https://en.wikipedia.org/wiki/Netpbm).

Mutiple examples of the output are laid out in atlas/texturemap form in the [atlas.png](atlas.png) file.

The program itself will generate a 3x3 atlas of the triangle from `n = 0` to `n = 8`.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Usage](#usage)
- [License](#license)

## Introduction

The [Koch Snowflake](https://en.wikipedia.org/wiki/Koch_snowflake) is a fractal named after Helge von Koch. It is formed by iteratively subdividing an equilateral triangle's sides into smaller equilateral triangles. This program provides a simple and customizable way to draw both verisons of it.

The AntiSnowflake is obtained by scaling the vector that points to the tip of the spikes by -1.

## Usage

1. **Clone:**

```bash
    git clone ssh://git@ilsrv.com:6722/Amat/KochSnowflakeGenerator.git
```

3. **Compile and run:**

The program takes no arguments, just run it normally:
```bash
cargo run --release
```

It is recommended that you use [this workaround in `.cargo/config.toml`](https://github.com/rust-lang/cargo/issues/2078) to authenticate in order to download the image drawing library (that I also wrote).

This will save an appropriately named file with a `.pbm` extension. I suggest that you convert it to a png for reasonable storage and viewing reasons using `ImageMagick`:

```bash
covert your_file.pbm out.png
```


## License

This project and all files contained therein and licensed under the [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.txt) license (see [COPYING](COPYING) file).
