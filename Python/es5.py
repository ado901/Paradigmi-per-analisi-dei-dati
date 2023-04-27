from typing import List


def merge(v1:list,v2:list)-> list:
    v1,v2 = sorted(v1), sorted(v2)
    b1, e1, b2, e2= 0, len(v1), 0, len(v2)
    while b1 < e1 or b2 < e2: 
        if b1 < e1 and (b2 == e2 or v1[b1]<= v2[b2]): 
            yield v1[b1]; b1 += 1
        else: yield v2[b2]; b2 += 1
        
print(list(merge([1,2,3,5,1,7,9],[2,6,9,15])))