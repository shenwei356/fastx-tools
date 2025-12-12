# fastx-tools

fastx-tools is a toolkit for FASTA/Q file manipulation, for learning purposes.


## Benchmarking

### Reading and writing plain FASTA/Q files

Note that, `seqkit` parses sequence ID from the header line, while other tools do not.

    in=t.human.fa
    # in=t.fq.gz

    hyperfine --warmup 3 --export-markdown - \
        "seqtk seq $in > /dev/null" \
        "seqkit seq -w 0 $in > /dev/null" \
        "fastx-tools seq $in > /dev/null" \
        "fastx-tools seq-needletail $in > /dev/null" 

A human T2T genome

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.human.fna > /dev/null` | 1.204 ± 0.010 | 1.185 | 1.216 | 1.00 |
| `seqkit seq -w 0 t.human.fna > /dev/null` | 1.355 ± 0.019 | 1.332 | 1.398 | 1.13 ± 0.02 |
| `fastx-tools seq t.human.fna > /dev/null` | 1.512 ± 0.014 | 1.488 | 1.536 | 1.26 ± 0.02 |
| `fastx-tools seq-needletail t.human.fna > /dev/null` | 2.096 ± 0.021 | 2.073 | 2.132 | 1.74 ± 0.02 |

5M SE-150bp reads.

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.fq > /dev/null` | 1.319 ± 0.030 | 1.291 | 1.381 | 2.17 ± 0.07 |
| `seqkit seq -w 0 t.fq > /dev/null` | 0.970 ± 0.021 | 0.942 | 0.996 | 1.60 ± 0.05 |
| `fastx-tools seq t.fq > /dev/null` | 0.816 ± 0.016 | 0.803 | 0.854 | 1.34 ± 0.04 |
| `fastx-tools seq-needletail t.fq > /dev/null` | 0.608 ± 0.013 | 0.597 | 0.640 | 1.00 |

Gzip-compressed 5M SE-150bp reads.

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.fq.gz > /dev/null` | 4.877 ± 0.041 | 4.821 | 4.958 | 1.56 ± 0.02 |
| `seqkit seq -w 0 t.fq.gz > /dev/null` | 4.914 ± 0.032 | 4.864 | 4.958 | 1.57 ± 0.02 |
| `fastx-tools seq t.fq.gz > /dev/null` | 3.333 ± 0.033 | 3.286 | 3.388 | 1.07 ± 0.01 |
| `fastx-tools seq-needletail t.fq.gz > /dev/null` | 3.123 ± 0.023 | 3.097 | 3.167 | 1.00 |
