# fastx-tools

fastx-tools is a toolkit for FASTA/Q file manipulation, for learning purposes.


## Benchmarking

### Reading and writing plain FASTA/Q files

Versions
    
    seqtk      1.5-r133
    seqkit     2.14.0
    fastx      0.1.3  (used by this tool)
    needletail 0.7.3

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
| `seqtk seq t.human.fna > /dev/null` | 1.063 ± 0.028 | 1.035 | 1.130 | 1.02 ± 0.03 |
| `seqkit seq -w 0 t.human.fna > /dev/null` | 1.637 ± 0.014 | 1.620 | 1.664 | 1.58 ± 0.02 |
| `fastx-tools seq t.human.fna > /dev/null` | 1.039 ± 0.012 | 1.023 | 1.059 | 1.00 |
| `fastx-tools seq-needletail t.human.fna > /dev/null` | 2.193 ± 0.315 | 1.842 | 2.724 | 2.11 ± 0.30 |

Peak memory

    seqtk seq                   253.49 MB
    seqkit seq                    1.23 GB   xxx
    fastx-tools seq             253.86 MB
    fastx-tools seq-needletail  524.44 MB   x

A human T2T genome (single-line FASTA)

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.human.single-line.fna > /dev/null` | 932.5 ± 14.7 | 917.3 | 965.2 | 1.02 ± 0.03 |
| `seqkit seq -w 0 t.human.single-line.fna > /dev/null` | 1405.3 ± 35.6 | 1354.2 | 1452.6 | 1.54 ± 0.05 |
| `fastx-tools seq t.human.single-line.fna > /dev/null` | 910.1 ± 17.9 | 884.5 | 929.0 | 1.00 |
| `fastx-tools seq-needletail t.human.single-line.fna > /dev/null` | 1170.1 ± 13.9 | 1144.1 | 1193.2 | 1.29 ± 0.03 |

Peak memory

    seqtk seq                    254.5 MB
    seqkit seq                  971.27 MB  xxx
    fastx-tools seq             253.94 MB
    fastx-tools seq-needletail  256.82 MB

5M SE-150bp reads.

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.fq > /dev/null` | 1.118 ± 0.013 | 1.102 | 1.140 | 1.85 ± 0.04 |
| `seqkit seq -w 0 t.fq > /dev/null` | 0.846 ± 0.010 | 0.832 | 0.860 | 1.40 ± 0.03 |
| `fastx-tools seq t.fq > /dev/null` | 0.704 ± 0.014 | 0.690 | 0.727 | 1.17 ± 0.03 |
| `fastx-tools seq-needletail t.fq > /dev/null` | 0.603 ± 0.009 | 0.589 | 0.618 | 1.00 |

Gzip-compressed 5M SE-150bp reads.

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.fq.gz > /dev/null` | 4.355 ± 0.025 | 4.317 | 4.394 | 1.47 ± 0.02 |
| `seqkit seq -w 0 t.fq.gz > /dev/null` | 4.267 ± 0.031 | 4.228 | 4.316 | 1.44 ± 0.02 |
| `fastx-tools seq t.fq.gz > /dev/null` | 2.990 ± 0.033 | 2.960 | 3.069 | 1.01 ± 0.02 |
| `fastx-tools seq-needletail t.fq.gz > /dev/null` | 2.959 ± 0.039 | 2.926 | 3.054 | 1.00 |
