# <div align="center">lperft</div>

<div align="center">

[![Build][build-badge]][build-link]
[![Version][crates-badge]][crates-link]
[![Release][release-badge]][release-link]
[![License][license-badge]][license-link]

</div>

**lperft** is a blazingly fast, multithreaded [perft][perft-link] tool designed for command-line use, ideal for debugging chess [move generators][laura-link]. It calculates the total number of nodes from a given chess position and outputs a list of all legal moves, along with their respective node counts at the specified depth.


## Installation and Compilation

You have several options for installing **lperft**:

### 1. Install using Cargo

The easiest way to install **lperft** is by using [Cargo][cargo-link], the Rust package manager. In your terminal, run the following command:

```bash
cargo install lperft
```
This will download and compile the latest version of **lperft** from [crates.io][crates-link]. 

### 2. Download Precompiled Binary

If you prefer not to compile the program yourself, you can download a precompiled binary from the [Releases][release-link] section of the repository. Simply download the appropriate version for your system and run the binary directly.

### 3. Compile from Source

If you want to compile **lperft** natively for your processor, make sure you have [Rust][rust-link] installed. Then, you can clone the repository and build the project with the following commands:

``` bash
git clone https://github.com/HansTibberio/lperft.git
cd lperft
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

For even better performance, you can enable the `bmi2` feature by using the following command to build with this optimization:

``` bash
RUSTFLAGS="-C target-cpu=native" cargo build --release --features bmi2
```
With any of these methods, you'll be able to run **lperft** directly from the command line once the installation or build process is complete.

## Usage

**lperft** is executed from the command line as follows:
```bash
lperft [OPTIONS] --depth <DEPTH>
```

### Example Usage

To run **lperft** on the default starting position with a search depth of 7 and use 2 threads:

```bash
lperft --depth 7 --threads 2
```
To analyze a custom position using a FEN string (e.g., a specific position from a game), set the depth to 6, and use a 128MB transposition table:

```bash
lperft --fen "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1" --depth 6 --hash 128
```

### Available Options

-   **`-f, --fen <FEN>`**
    Specifies the position in [FEN][fen-link] format (enclosed in quotes).  
    If omitted, the default starting position is used:  
    `"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"`
    
-   **`-d, --depth <DEPTH>`**  
    The search depth to evaluate.
    
-   **`-H, --hash <HASH>`**  
    The size of the transposition table in megabytes (optional).  
    If not specified, the search will proceed without a transposition table.
    
-   **`-t, --threads <THREADS>`**  
    The number of threads to use for parallel node search.  
    By default, **1** thread is used.

## Benchmarks
Below are the benchmarks for **lperft**, measuring its performance on **Perft(7), Perft(8), and Perft(9)** from the **starting position** and the well-known _Kiwipete_ position.

**Starting Position Benchmarks**
|Depth|Hash (MB)|Threads|Time|NPS|
|:-:|:-:|:-:|-|-|
|7|0|1|5.052 s|632,580,821|
|7|0|2|2.606 s|1,226,088,379|
|7|64|1|1.655 s|1,930,084,154|
|7|128|1|1.620 s|1,972,545,265|
|7|64|2|845 ms|3,780,542,314|
|7|128|2|825 ms|3,870,346,038|
|8|1024|8|4.087 s|20,794,666,969|
|8|1024|12|3.192 s|26,624,967,590|
|9|8192|12|37.560 s|64,949,336,635|

**Kiwipete Benchmarks**
|Depth|Hash (MB)|Threads|Time|NPS|
|:-:|:-:|:-:|-|-|
|7|4096|12|10.294 s|36,348,767,805|
|8|8192|12|247.768 s|62,533,872,605|

**Test system:** AMD Ryzen 5 5600G (3.9 GHz), 32 GB DDR4 3200 MHz, Windows 10

## License

This project is licensed under **GPLv3**. See the [LICENSE][license-link] file for details.

[build-badge]:https://img.shields.io/github/actions/workflow/status/HansTibberio/lperft/build.yml
[build-link]:https://github.com/HansTibberio/lperft/actions/workflows/build.yml
[license-link]:https://github.com/hanstibberio/lperft/blob/master/LICENSE
[license-badge]:https://img.shields.io/github/license/hanstibberio/lperft?label=license&color=success
[crates-link]:https://crates.io/crates/lperft
[crates-badge]:https://img.shields.io/crates/v/lperft
[release-badge]:https://img.shields.io/github/v/release/HansTibberio/lperft?label=official%20release
[release-link]:https://github.com/HansTibberio/lperft/releases/latest

[cargo-link]:https://doc.rust-lang.org/cargo/
[fen-link]:https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
[rust-link]:https://www.rust-lang.org/
[perft-link]:https://www.chessprogramming.org/Perft
[laura-link]:https://github.com/HansTibberio/Laura/tree/master/laura_core
