# fastx-tools

fastx-tools is a toolkit for FASTA/Q file manipulation, for learning purposes.


## Benchmarking

### Reading and writing plain FASTA/Q files

A human T2T genome

    in=t.human.fa

    hyperfine --warmup 3 --export-markdown - \
        "seqtk seq $in > /dev/null" \
        "seqkit seq -w 0 $in > /dev/null" \
        "fastx-tools seq $in > /dev/null" \
        "fastx-tools seq-needletail $in > /dev/null" 

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.human.fna > /dev/null` | 1.169 ± 0.013 | 1.146 | 1.189 | 1.00 |
| `seqkit seq -w 0 t.human.fna > /dev/null` | 1.429 ± 0.041 | 1.380 | 1.489 | 1.22 ± 0.04 |
| `fastx-tools seq t.human.fna > /dev/null` | 1.402 ± 0.053 | 1.366 | 1.544 | 1.20 ± 0.05 |
| `fastx-tools seq-needletail t.human.fna > /dev/null` | 1.968 ± 0.026 | 1.942 | 2.023 | 1.68 ± 0.03 |

Gzip-compressed 1M SE-150bp reads.

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `seqtk seq t.fq.gz > /dev/null` | 953.0 ± 5.7 | 940.5 | 959.8 | 1.52 ± 0.02 |
| `seqkit seq -w 0 t.fq.gz > /dev/null` | 1002.9 ± 25.9 | 978.4 | 1068.7 | 1.60 ± 0.05 |
| `fastx-tools seq t.fq.gz > /dev/null` | 713.3 ± 27.5 | 685.6 | 769.8 | 1.13 ± 0.05 |
| `fastx-tools seq-needletail t.fq.gz > /dev/null` | 628.5 ± 9.4 | 617.2 | 644.9 | 1.00 |
