"""
Test code to examine turning a kmer to index
"""
import sys
import numpy as np

letters = list('ATCG')
comp_letters = list('TAGC')

def generate_kmers(char_list, k):
    import itertools
    kmers = list(itertools.product(char_list, repeat=k))
    kmers = ["".join(kmer) for kmer in kmers]
    return kmers

def do(kmer, m_letters):
    k = len(kmer)
    n = 4**k
    mb = 4 ** np.arange(k-1, -1, -1)
    digits = np.array([m_letters.index(_) for _ in kmer])
    numbering = (digits * mb).sum()
    print(numbering)
    kfeat = np.zeros(n)
    
    kfeat[numbering] += 1
    print(kfeat)

ksize = int(sys.argv[1]) if len(sys.argv) != 1 else 3

for kmer in generate_kmers(letters, ksize):
    print(kmer)
    sys.stdout.write("+ ")
    do(kmer, letters)
    sys.stdout.write("- ")
    do(kmer, comp_letters)
    sys.stdout.write("r ")
    do(kmer[::-1], letters)

