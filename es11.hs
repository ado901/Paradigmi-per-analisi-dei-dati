maxChange:: [Int] -> Int
maxChange xs = fst (foldl (\(counter,max) x -> if x>max then (counter+1,x) else (counter,max)) (0,0) xs)
