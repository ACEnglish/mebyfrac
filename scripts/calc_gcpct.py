import sys
import pysam
f = pysam.FastaFile(sys.argv[1])

tl = 0
gc = 0
for s in f.references:
    s = f[s]
    tl += len(s)
    gc += s.count('G') + s.count('C')
print(gc / tl)
