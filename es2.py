import itertools
from random import shuffle
values = list(range(1, 6))
shuffle(values)
print(f'Lista: {values}')
def count_tops(values: list[int]) -> int:
    currentmax=0
    for i in values:
        if i>currentmax:
            currentmax=i
            yield currentmax
print(list(count_tops(values)))

class Tops():
    def __init__(self, values: list[int]):
        self.values = values
    def __iter__(self):
        
        self._currentmax=0
        self._currentindex=0
        return self
    def __next__(self):
        if len(self.values)<=self._currentindex:
            raise StopIteration
        if self._currentmax<self.values[self._currentindex]:
            self._currentmax=self.values[self._currentindex]
            self._currentindex=self._currentindex+1
            return self._currentmax 
        self._currentindex=self._currentindex+1
        return self.__next__()
       
        

print(list(Tops(values)))