# fastx-tools

fastx-tools is a toolkit for FASTA/Q file manipulation, for learning purposes.
It uses [fastseq](https://github.com/shenwei356/fastseq) for FASTA/Q file parsing.

## Benchmarking

Tools and Versions
    
- [`seqtk`](https://github.com/lh3/seqtk) 1.5-r133
- [`seqkit`](https://github.com/shenwei356/seqkit/) 2.13.0
- `fastx-tools seq` (this tool), using [fastseq](https://github.com/shenwei356/fastseq) 0.1.4
- `fastx-tools seq-needletail` (this tool), using [needletail](https://github.com/onecodex/needletail) 0.7.3

### Reading and writing plain FASTA/Q files

Note that, `seqkit` parses sequence ID from the header line, while other tools do not.

    in=t.human.fna
    # in=t.human.single-line.fna
    # in=t.fq
    # in=t.fq.gz

    hyperfine --warmup 3 --export-markdown - \
        "seqtk seq $in > /dev/null" \
        "seqkit seq -w 0 $in > /dev/null" \
        "fastx-tools seq $in > /dev/null" \
        "fastx-tools seq-needletail $in > /dev/null"


A human T2T genome

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.human.fna > /dev/null` | 1.012 ± 0.023 | 0.987 | 1.049 | 1.06 ± 0.02 |
| `seqkit seq -w 0 t.human.fna > /dev/null` | 1.134 ± 0.019 | 1.104 | 1.157 | 1.19 ± 0.02 |
| `fastx-tools seq t.human.fna > /dev/null` | 0.955 ± 0.004 | 0.950 | 0.960 | 1.00 |
| `fastx-tools seq-needletail t.human.fna > /dev/null` | 1.760 ± 0.023 | 1.729 | 1.799 | 1.84 ± 0.03 

Peak memory

    seqtk seq                   252.97 MB
    seqkit seq                    1.23 GB   xxx
    fastx-tools seq             254.55 MB
    fastx-tools seq-needletail  415.41 MB   x

A human T2T genome (single-line FASTA)

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.human.single-line.fna > /dev/null` | 801.7 ± 17.3 | 783.3 | 826.6 | 1.04 ± 0.02 |
| `seqkit seq -w 0 t.human.single-line.fna > /dev/null` | 974.7 ± 10.1 | 966.2 | 992.2 | 1.26 ± 0.01 |
| `fastx-tools seq t.human.single-line.fna > /dev/null` | 773.1 ± 3.4 | 768.4 | 779.3 | 1.00 |
| `fastx-tools seq-needletail t.human.single-line.fna > /dev/null` | 1017.3 ± 5.5 | 1011.2 | 1026.7 | 1.32 ± 0.01 |

Peak memory

    seqtk seq                   252.48 MB
    seqkit seq                  973.00 MB  xxx
    fastx-tools seq             253.80 MB
    fastx-tools seq-needletail  256.58 MB

5M plain SE-150bp reads.

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.fq > /dev/null` | 1.056 ± 0.009 | 1.045 | 1.074 | 1.89 ± 0.02 |
| `seqkit seq -w 0 t.fq > /dev/null` | 0.791 ± 0.010 | 0.781 | 0.807 | 1.42 ± 0.02 |
| `fastx-tools seq t.fq > /dev/null` | 0.661 ± 0.012 | 0.651 | 0.692 | 1.18 ± 0.02 |
| `fastx-tools seq-needletail t.fq > /dev/null` | 0.559 ± 0.003 | 0.554 | 0.564 | 1.00 |

Peak memory

    seqtk seq                     9.92 MB
    seqkit seq                   87.25 MB  xxx
    fastx-tools seq              10.75 MB
    fastx-tools seq-needletail   10.69 MB


### Reading and writing gzip-compressed FASTA/Q files

A gzip-compressed human T2T genome

    in=t.human.fna.gz
    # in=t.fq.gz
    
    hyperfine --warmup 3 --export-markdown - \
        "seqtk seq $in > /dev/null" \
        "seqkit seq -w 0 $in > /dev/null" \
        "fastx-tools seq $in > /dev/null" \
        "fastx-tools seq-needletail $in > /dev/null"
        
| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.human.fna.gz > /dev/null` | 8.572 ± 0.015 | 8.559 | 8.606 | 2.01 ± 0.02 |
| `seqkit seq -w 0 t.human.fna.gz > /dev/null` | 10.978 ± 0.027 | 10.936 | 11.032 | 2.57 ± 0.02 |
| `fastx-tools seq t.human.fna.gz > /dev/null` | 4.267 ± 0.032 | 4.224 | 4.322 | 1.00 |
| `fastx-tools seq-needletail t.human.fna.gz > /dev/null` | 4.805 ± 0.011 | 4.792 | 4.829 | 1.13 ± 0.01 |

Gzip-compressed 5M SE-150bp reads (write plain format)

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.fq.gz > /dev/null` | 4.478 ± 0.173 | 4.299 | 4.747 | 2.34 ± 0.09 |
| `seqkit seq -w 0 t.fq.gz > /dev/null` | 4.182 ± 0.025 | 4.159 | 4.235 | 2.19 ± 0.02 |
| `fastx-tools seq t.fq.gz > /dev/null` | 1.914 ± 0.009 | 1.902 | 1.929 | 1.00 |
| `fastx-tools seq-needletail t.fq.gz > /dev/null` | 1.922 ± 0.007 | 1.911 | 1.930 | 1.00 ± 0.01 |

Gzip-compressed 5M SE-150bp reads (write gzip).
Note that Seqkit uses the compression level of 5 in writting gzip files.

    in=t.fq.gz
    
    hyperfine --warmup 3 --export-markdown - \
        "seqtk seq $in | pigz -c > t.seqtk.gz" \
        "seqkit seq -w 0 $in -o t.seqkit.gz" \
        "fastx-tools seq $in -o t.fastx.gz" \
        "fastx-tools seq-needletail $in -o t.needletail.gz"

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.fq.gz \| pigz -c > t.seqtk.gz` | 14.337 ± 0.053 | 14.266 | 14.457 | 3.21 ± 0.02 |
| `seqkit seq -w 0 t.fq.gz -o t.seqkit.gz` | 4.852 ± 0.049 | 4.764 | 4.908 | 1.09 ± 0.01 |
| `fastx-tools seq t.fq.gz -o t.fastx.gz` | 4.470 ± 0.029 | 4.400 | 4.512 | 1.00 |
| `fastx-tools seq-needletail t.fq.gz -o t.needletail.gz` | 4.511 ± 0.032 | 4.450 | 4.552 | 1.01 ± 0.01 |

## License

[MIT License](https://github.com/shenwei356/fastx-tools/blob/master/LICENSE)
