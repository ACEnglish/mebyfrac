# Introduction

This project serves two purposes. First, this is an exploration into a [Paper](https://arxiv.org/pdf/1112.1528.pdf).
Second, I'm trying to learn Rust.

# Quick Start

run `mebyfrac <k> <fasta>` where `k` is the kmer size and `fasta` is a fasta file

# TODOs:

### Code
- HashMap is slow. Copy the array counting from kmer.py.
- how to build (`cargo build`)
- probably will need to make a per-os release 
- Figure out crosscomp
 - rustup target add x86_64-apple-ios
 - rustup target add x86_64-apple-darwin
 - https://wapl.es/rust/2019/02/17/rust-cross-compile-linux-to-macos.html
- Clean code
 - I have some trash in there
- Better argparsing (e.g. a -h)
- Better output
- Single parsing for multiple kmers

## Table 1: Math Table for k=3. The Generating set is the first column. 

| Class | g   | C(R(g)) | C(g) | R(g) |
|-------|-----|---------|------|------|
| 1     | AAA | -       | TTT  | -    |
| 2     | AAT | ATT     | TTA  | TAA  |
| 3     | TTG | CAA     | AAC  | GTT  |
| 4     | CTT | AAG     | GAA  | TTC  |
| 5     | ATA | -       | TAT  | -    |
| 6     | ATC | GAT     | TAG  | CTA  |
| 7     | ATG | CAT     | TAC  | GTA  |
| 8     | ACA | -       | TGT  | -    |
| 9     | TGA | TCA     | ACT  | AGT  |
| 10    | CCA | TGG     | GGT  | ACC  |
| 11    | GCA | TGC     | CGT  | ACG  |
| 12    | TCT | -       | AGA  | -    |
| 13    | GCT | AGC     | CGA  | TCG  |
| 14    | AGG | CCT     | TCC  | GGA  |
| 15    | CAC | -       | GTG  | -    |
| 16    | CAG | CTG     | GTC  | GAC  |
| 17    | CTC | -       | GAG  | -    |
| 18    | CCC | -       | GGG  | -    |
| 19    | GCC | GCC     | CGG  | CCG  |
| 20    | GCG | -       | CGC  | -    |

sum(F(g)) + sum(F(rg)) ==  0.5

## Some numbers


data/R_sphaeroides.fa
GC% - 0.6878748050210077
