import sys
import pysam

v = pysam.FastaFile(sys.argv[1])
all_seq = ""
for ref in v.references:
    all_seq += v[ref]

import random
tra = str.maketrans('ATCG', 'TAGC')

# four big inversions
for i in range(4):
    pos = random.randint(0, len(all_seq)) - 1000
    all_seq = all_seq[:pos] + all_seq[pos:pos+ 1000].translate(tra)[::-1] + all_seq[pos+1000:]

print(">withinv")
print(all_seq)
