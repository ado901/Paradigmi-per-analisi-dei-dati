import Data.List
import System.Random
import GHC.IO
import Control.Monad
import Data.Word (Word32)
import Data.Bits (xor, shiftL, shiftR)
import Data.Time.Clock.POSIX (getPOSIXTime)
import Control.Monad.Trans.RWS (put)


{- Write a pure function in Haskell

    (Int, Int) -> [Bool] -> Int -> [Bool]
    The first pair represents the number of columns and rows
    The list of bools represents a matrix
    The integer indicates the cell to play on
    In the result matrix, the chosen cell does not change its value
    The 4 adjacent cells (wrt the chosen one, Δx + Δy = 1) flip their values
 -}
chunksOf :: Int -> [a] -> [[a]]
chunksOf n [] = []
chunksOf n xs = a : chunksOf n b where
  (a, b) = splitAt n xs

cleanup :: (Int, Int) -> [Bool] -> Int -> [Bool]
cleanup (rows,cols) matrix index=[if (abs(x-i)+abs(y-j))==1 then not (matrix!!(i*cols+j)) else matrix!!(i*cols+j) | let (x,y)=divMod index cols ,i<-[0..rows-1], j<-[0..cols-1]]
{- Prepare a clean w×h matrix and play m random moves

    w, h, m are configurable
    Use the previous “clean-up” function, to play a move
    Possibly, avoid repeating the moves (play m different moves)

Then, allow the user to play

    Check if the matrix is completely cleaned up (solved)
    Count user's moves

Minimize the impure part  -}

{- IMPORTANTE PER IL PROF 
l'esercizio l'ho interpretato come un usare la parte randomica come base per quella utente (infatti originariamente
controllavo ogni mossa randomica per la vittoria) infatti nella versione consegnata precedentemente c'era solo la parte utente, reimplementarla ora è
stato banale-}

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

randommoves :: Int -> Int -> Int -> [Bool] -> IO [Bool]
randommoves m cols rows matrix= do
    gen <- getRng32
    return (foldl (cleanup (rows, cols)) matrix (take m $ nub $ randints (0,rows*cols-1) gen))
main :: IO ()
main= do
    
    (rows,cols,moves) <-getrowscols
    let matrix= replicate (rows*cols) False
    mapM_ print (chunksOf cols matrix)
    newmatrix <- randommoves moves cols rows matrix
    putStrLn "Random moves"
    mapM_ print (chunksOf cols newmatrix)
    play rows cols newmatrix 0
    
    
check :: [Bool] -> Bool
check = and


play :: (Show t, Num t) => Int -> Int -> [Bool] -> t -> IO ()
play rows cols matrix moves= do
    putStrLn "Insert index"
    guess <- getLine
    when (guess /= "\n") $ do
        let newmatrix= cleanup (rows,cols) matrix (read guess :: Int)
        mapM_ print (chunksOf cols newmatrix)
        if check newmatrix then print ("solved with"++show (moves+1)) else play rows cols newmatrix (moves+1)

    
getrowscols :: IO (Int,Int,Int)
getrowscols = do
    putStrLn "Insert rows and columns"
    [rows, cols] <- sequence [getLine, getLine]
    putStrLn "Insert number of random moves"
    moves <- getLine
    return (read rows :: Int, read cols :: Int, read moves :: Int)