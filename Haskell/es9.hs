massimo1:: [Int] -> Int -> Int
massimo1 [] n = n
massimo1 (x:xs) n
  | x > n = massimo1 xs x
  | otherwise = massimo1 xs n

massimo2:: [Int] -> Int-> Int
massimo2 [] n = n
massimo2 (x:xs) n= if x > n then massimo2 xs x else massimo2 xs n

massimo3:: [Int] -> Int-> Int
massimo3 [] n = n
massimo3 (x:xs) n= max x (massimo3 xs n)