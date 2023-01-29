import sys
import random


seq_len = int(sys.argv[1])
if len(sys.argv) > 2:
    letters = list(sys.argv[2])
else:
    letters = list("ATGC")

if len(sys.argv) > 3:
    seed = int(sys.argv[3])
else:
    seed = random.randint(0, 1020292093)
random.seed(seed)

with open('/dev/stdout', 'w') as fout:
    fout.write(f">random {seq_len} {''.join(letters)} {seed}\n")
    for i in range(seq_len):
        fout.write(random.choice(letters))
