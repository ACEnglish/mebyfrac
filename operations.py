import math
import doctest

def R(w):
    """
    Reverse operator
    
    Example:
        >>> w = "ATGT"
        >>> R(w) == "TGTA"
        True
    """
    return w[::-1]

COMPL = str.maketrans("ATCGN", "TAGCN")
def C(w):
    """
    Compliment operator
    
    Example:
        >>> w = "ATGT"
        >>> C(w) == "TACA"
        True
    """
    return w.translate(COMPL)


def is_involution(w, op):
    """
    An operator is an involution when its inverse operator is the operator itself
    
    Example:
        >>> w = "ATC"
        >>> C(C(w)) == w
        True
        >>> R(R(w)) == w
        True
    """
    return op(op(w)) == w


def bell_number(n: int) -> int:
    """
    Calculates the bell number of a set of length n using recursion.

    Examples:
    >> test_cases = [
    ...     (0, 1), (1, 1), (2, 2), (3, 5), (4, 15), (5, 52),
    ...     (6, 203), (7, 877), (8, 4140), (9, 21147), (10, 115975)
    ... ]
    >> for n, expected in test_cases:
    ...     assert bell_number(n) == expected, f"{bell_number(n)} != {expected}"
    """
    s = [[0 for _ in range(n+1)] for _ in range(n+1)]
    for i in range(n+1):
        for j in range(n+1):
            if j > i:
                continue
            elif(i==j):
                s[i][j] = 1
            elif(i==0 or j==0):
                s[i][j]=0
            else:
                s[i][j] = j*s[i-1][j] + s[i-1][j-1]
    ans = 0
    for i in range(0,n+1):
        ans+=s[n][i]
    return ans

if __name__ == "__main__":
    import doctest
    doctest.testmod()

