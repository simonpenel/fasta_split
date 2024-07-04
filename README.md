# Split a fasta file into several fasta files

https://crates.io/crates/fasta_split

https://github.com/simonpenel/fasta_split/


# Install

`cd fasta_split`

`cargo build --release` 


or
`cargo install fasta_split`


# Run

`target/release/fasta-split -h`

or

`fasta-split -h `


# Usage

```

Usage: fasta_split [OPTIONS] <FASTA_FILE> <FAM_FILE>

Arguments:
  <FASTA_FILE>  FASTA INPUT (file path or stdin)
  <FAM_FILE>    FAM_FILE INPUT (file path)

Options:
      --orphans  Include orphan families
  -h, --help     Print help
  -V, --version  Print version
```


# Family file format

A tabular file:

```FAM1  seqname1
FAM1  seqname2
FAM1  seqname3
FAM1  seqname4
...
FAM10  seqname101
FAM10  seqname102
FAM10  seqname103
FAM10  seqname104`
```
