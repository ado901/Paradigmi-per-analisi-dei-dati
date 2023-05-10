import Data.Array
import System.Random
import Data.Word (Word32)
import Data.Bits (xor, shiftL, shiftR)
import Data.Time.Clock.POSIX (getPOSIXTime)
import Data.List (nub)
{- 
    Modify the “Clean-up game” project
    Use a Haskell array instead of a list
    Choose whether to use as index:
        A single Int, or…
        A pair of Ints

    FAQ:
    -lo so, ho capito male la traccia e me ne sono accorto a metà, ma intanto che c'ero sono andato fino in fondo
 -}
type Rng32 = Word32

xorshift32 :: Rng32 -> Rng32
xorshift32 a = d where
  b = a `xor` (a `shiftL` 13)
  c = b `xor` (b `shiftR` 17)
  d = c `xor` (c `shiftL` 5)

randint :: (Int, Int) -> Rng32 -> (Int, Rng32)
randint (nmin, nmax) gen = (val, nxt) where
    nxt = xorshift32 gen
    val = nmin + fromIntegral nxt `mod` (nmax + 1 - nmin)

randints :: (Int, Int) -> Rng32 -> [Int]
randints range gen =
    val : randints range nxt
    where (val, nxt) = randint range gen

getRng32 :: IO Rng32
getRng32 = do
    now <- getPOSIXTime
    return (round (now * 1000) :: Rng32)
randommoves :: Int -> Int -> Int -> Array Int Bool -> IO (Array Int Bool)
randommoves m cols rows matrix= do
    gen <- getRng32
    return (foldl (cleanup (rows, cols)) matrix (take m $ nub $ randints (0,rows*cols-1) gen))
cleanup :: (Int, Int) -> Array Int Bool -> Int -> Array Int Bool
cleanup (rows,cols) matrix index= matrix// [(i*cols+j,not $ matrix!(i*cols+j))| let (x,y)=divMod index cols,i<-[0..rows-1], j <- [0..cols-1], abs(i-x)+abs(j-y)==1]

main :: IO ()
main= do
    
    (rows,cols, m) <-getrowscols
    let matrix = array (0,rows*cols-1) [(i,False) | i<-[0..rows*cols-1]]
    mapM_ print (chunksOf rows (elems matrix))
    newmatrix <- randommoves m cols rows matrix
    putStrLn "Random moves"
    mapM_ print (chunksOf rows (elems newmatrix))
    play rows cols newmatrix 0
    
check :: Array Int Bool -> Bool
check = and

play :: (Show a, Num a) => Int -> Int -> Array Int Bool -> a -> IO ()
play rows cols matrix moves= do
    putStrLn "Insert index"
    guess <- checkplayermove
    let newmatrix= checkval rows cols matrix guess
    mapM_ print (chunksOf rows (elems newmatrix)) 
    if check newmatrix then print ("solved with"++show (moves+1)) else play rows cols newmatrix (moves+1)

    
getrowscols :: IO (Int,Int, Int)
getrowscols = do
    putStrLn "Insert rows and columns"
    [rows, cols] <- sequence [getLine, getLine]
    putStrLn "Insert number of random moves"
    m <- getLine
    return (read rows :: Int, read cols :: Int, read m::Int)


chunksOf :: Int -> [a] -> [[a]]
chunksOf n [] = []
chunksOf n xs = a : chunksOf n b where
  (a, b) = splitAt n xs

{- modify chunksOf to have a function with an array monodimensional as parameter and an array bidimentional as output, which each row is of length n -}
chunksOfArray :: Int -> Array Int a -> Array (Int, Int) a
chunksOfArray n xs= array ((0,0),divMod (snd (bounds xs)) n) [((x,y), xs!i) | i<-[0..snd (bounds xs)], let (x,y)=divMod i n]

checkplayermove :: IO (Either (Int, Int) Int)
checkplayermove= do
    guess <- getLine
    if guess /= "\n" then do
        let word = words guess
        if length word==2 then do
            return (Left (read (head word) :: Int, read (word!!1) :: Int))
        else 
            return(Right (read guess :: Int))
            
    else checkplayermove


checkval :: Int-> Int -> Array Int Bool -> Either (Int, Int) Int -> Array Int Bool
checkval rows cols matrix (Left guess)= cleanup (rows,cols) matrix $ fst guess * cols + snd guess
checkval rows cols matrix (Right guess)= cleanup (rows,cols) matrix guess

