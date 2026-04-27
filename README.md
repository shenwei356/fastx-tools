# fastx-tools

fastx-tools is a toolkit for FASTA/Q file manipulation, for learning purposes.


## Benchmarking

### Reading and writing plain FASTA/Q files

Note that, `seqkit` and `fastx-tools seq`parses sequence ID from the header line, while other tools do not.

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
| `seqtk seq t.human.fna > /dev/null` | 1.196 ± 0.024 | 1.161 | 1.230 | 1.00 |
| `seqkit seq -w 0 t.human.fna > /dev/null` | 1.353 ± 0.024 | 1.324 | 1.402 | 1.13 ± 0.03 |
| `fastx-tools seq t.human.fna > /dev/null` | 1.243 ± 0.013 | 1.234 | 1.280 | 1.04 ± 0.02 |
| `fastx-tools seq-needletail t.human.fna > /dev/null` | 2.036 ± 0.033 | 1.977 | 2.096 | 1.70 ± 0.04 |

Peak memory

    seqtk seq                   253.49 MB
    seqkit seq                    1.23 GB   xxx
    fastx-tools seq             253.86 MB
    fastx-tools seq-needletail  524.44 MB   x

A human T2T genome (single-line FASTA)

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.human.single-line.fna > /dev/null` | 819.6 ± 10.5 | 805.8 | 840.3 | 1.00 |
| `seqkit seq -w 0 t.human.single-line.fna > /dev/null` | 1073.3 ± 23.8 | 1050.1 | 1104.1 | 1.31 ± 0.03 |
| `fastx-tools seq t.human.single-line.fna > /dev/null` | 835.5 ± 24.7 | 809.3 | 877.3 | 1.02 ± 0.03 |
| `fastx-tools seq-needletail t.human.single-line.fna > /dev/null` | 1078.7 ± 17.6 | 1062.8 | 1116.9 | 1.32 ± 0.03 |

Peak memory

    seqtk seq                    254.5 MB
    seqkit seq                  971.27 MB  xxx
    fastx-tools seq             253.94 MB
    fastx-tools seq-needletail  256.82 MB

5M SE-150bp reads.

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.fq > /dev/null` | 1.105 ± 0.011 | 1.093 | 1.125 | 1.80 ± 0.04 |
| `seqkit seq -w 0 t.fq > /dev/null` | 0.842 ± 0.009 | 0.832 | 0.861 | 1.37 ± 0.03 |
| `fastx-tools seq t.fq > /dev/null` | 0.760 ± 0.013 | 0.747 | 0.789 | 1.23 ± 0.03 |
| `fastx-tools seq-needletail t.fq > /dev/null` | 0.615 ± 0.013 | 0.604 | 0.645 | 1.00 |

Gzip-compressed 5M SE-150bp reads.

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.fq.gz > /dev/null` | 4.371 ± 0.019 | 4.346 | 4.402 | 1.45 ± 0.01 |
| `seqkit seq -w 0 t.fq.gz > /dev/null` | 4.238 ± 0.024 | 4.220 | 4.290 | 1.40 ± 0.01 |
| `fastx-tools seq t.fq.gz > /dev/null` | 3.076 ± 0.029 | 3.046 | 3.145 | 1.02 ± 0.01 |
| `fastx-tools seq-needletail t.fq.gz > /dev/null` | 3.018 ± 0.014 | 2.997 | 3.035 | 1.00 |
