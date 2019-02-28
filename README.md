# filtercov
Blazingly fast Rust program that filters bedGraph-like files for coverage.

## Compiling
Compile with the following command
```
cargo build --release
```
The binary will be located in `target/release/filtercov

## Running
Simply run
```
filtercov "lvl1 lvl2..." file1 file2...
```
A file will be produced for each input file at each level.

Example:
```
../target/release/filtercov "5 10 30" test1.bed test2.bed
[5, 10, 30]
["test1.bed", "test2.bed"]
test1.bed -> test1.5.bed
test2.bed -> test2.5.bed
test1.5.bed -> test1.10.bed
test2.5.bed -> test2.10.bed
test1.10.bed -> test1.30.bed
test2.10.bed -> test2.30.bed
```
