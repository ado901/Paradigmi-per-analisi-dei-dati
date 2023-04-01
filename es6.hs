reverseTr:: [a] ->[a] -> [a]
reverseTr [] ys = ys
reverseTr (x:xs) ys = reverseTr xs (x:ys)