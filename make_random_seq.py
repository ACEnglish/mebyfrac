import sys
import random


seq_len = int(sys.argv[1])
letters = list("ATCG")
with open('/dev/stdout', 'w') as fout:
    fout.write(f">random {seq_len}\n")
    for i in range(seq_len):
        fout.write(random.choice(letters))
