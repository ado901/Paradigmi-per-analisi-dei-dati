maxOfTwo:: (Ord a) => a -> a -> a
maxOfTwo x y
    |x <= y =y
    | otherwise = x

calculate:: (Num a, Show a, Fractional a, Eq a) => Char -> a -> a -> String
calculate '+' a b = show (a+b)
calculate '-' a b = show (a-b)
calculate '*' a b = show (a*b)

calculate '/' a 0 = "error"
calculate '/' a b = show (a/b)
calculate _ _ _ = "error"