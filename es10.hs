skyScraper1:: [Int] -> (Int, Int) -> Int
skyScraper1 [] (_ , count) = count
skyScraper1 (x:xs) (n, count)
  | x > n = skyScraper1 xs (x, count+1)
  | otherwise = skyScraper1 xs (n, count)