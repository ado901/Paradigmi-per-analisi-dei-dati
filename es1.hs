
takefirst5 xs=take 5 xs
lenght1 xs= fromIntegral(length xs)
average xs = sum (takefirst5 xs) / lenght1 (takefirst5 xs)