from functools import partial
from itertools import count, takewhile
from math import pow
def sumOddSquares():
    pow2= lambda x: pow(x,2)
    result=sum(takewhile(lambda x: x <10000, map(pow2,count(start=1, step=2))))
    return result
print (sumOddSquares())
def sumOddSquares2():
    pow2= lambda x: pow(x,2)
    result=sum(takewhile(lambda x: x<10000, (x**2 for x in count(start=1, step=2))))
    return result
print(sumOddSquares2())