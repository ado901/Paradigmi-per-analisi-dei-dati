from random import shuffle
values = list(range(1, 6))
shuffle(values)  # e.g., [3, 1, 4, 2, 5]
def count_tops(values: list[int]) -> int:
    maxcount=0
    currentmax=0
    for i in values:
        if i>currentmax:
            currentmax=i
            maxcount+=1
    return maxcount
            
print(count_tops(values))