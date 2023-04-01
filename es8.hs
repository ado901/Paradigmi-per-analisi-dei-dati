collatz :: (Integral a) => a -> [a]
collatz 1 = [1]
collatz n
  | even n = n : collatz (n `div` 2)
  | odd n = n : collatz (n * 3 + 1)

checklength= length(filter (>15)(map (length . collatz) [1..100]))