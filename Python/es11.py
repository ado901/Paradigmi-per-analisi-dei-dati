from functools import reduce
def skyScrapers(l):
    return reduce(lambda acc, x: (acc[0]+1,x) if x>acc[1] else (acc[0],acc[1]), l, (0,0))[0] # acc[0] è il contatore
print(skyScrapers([1,9,2,3,4,5,10]))	