import Data.List
import System.Random
import GHC.IO
import Control.Monad

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
main :: IO ()
main= do
    
    (rows,cols) <-getrowscols
    let matrix= replicate (rows*cols) False
    mapM_ print (chunksOf cols matrix)
    play rows cols matrix 0
    
    
check :: [Bool] -> Bool
check = and


play :: (Show t, Num t) => Int -> Int -> [Bool] -> t -> IO ()
play rows cols matrix moves= do
    putStrLn "Insert index"
    guess <- getLine
    when (guess /= "\n") $ do
        let newmatrix= cleanup (rows,cols) matrix (read guess :: Int)
        mapM_ print (chunksOf cols newmatrix)
        if check newmatrix then print ("solved with"++show moves) else play rows cols newmatrix (moves+1)

    
getrowscols :: IO (Int,Int)
getrowscols = do
    putStrLn "Insert rows and columns"
    [rows, cols] <- sequence [getLine, getLine]
    return (read rows :: Int, read cols :: Int)