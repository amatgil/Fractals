# (Anti) Koch Snowflake generator

This Rust program generates simple (Anti) Koch snowflakes from the commandline in the
[Netpbm format](https://en.wikipedia.org/wiki/Netpbm).

Mutiple examples of the output are laid out in atlas/texturemap form in the [atlas.png](atlas.png) file.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Usage](#usage)
- [License](#license)

## Introduction

The [Koch Snowflake](https://en.wikipedia.org/wiki/Koch_snowflake) is a fractal named after Helge von Koch. It is formed by iteratively subdividing an equilateral triangle's sides into smaller equilateral triangles. This program provides a simple and customizable way to draw both versoins of it.

The AntiSnowflake is obtained by rotating -τ/6 radians instead of τ/6 (easily visible in the code).

## Usage

1. **Clone:**

```bash
    git clone ssh://git@ilsrv.com:6722/Amat/KochSnowflakeGenerator.git
```

3. **Compile and run:**

The program expects two arguments:
- `n`: how many iterations to go through (natural number)
- `anti`: whether you want the standard version or the anti version ("0" or "1" respectively)

```bash
cargo run --release -- 6 0
```

It is recommended that you use [this workaround in `.cargo/config.toml`](https://github.com/rust-lang/cargo/issues/2078) to authenticate in order to download the image drawing library (that I also wrote).

This will save an appropriately named file with a `.ppm` extension. I suggest that you convert it to a png for reasonable storage and viewing reasons using `ImageMagick`:

```bash
covert your_file.ppm out.png
```


## License

This project and all files contained therein and licensed under the [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.txt) license (see COPYING file).
