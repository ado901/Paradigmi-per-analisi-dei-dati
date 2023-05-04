import System.IO
import Data.List ( nub, transpose )
{- 
    Read one of the following files in Haskell
        skyscrapers_games.zip
        Represent data as a list of lists of ints, i.e., [[Int]]
        The numbers on the borders are the *constraints* to satisfy
    Check if data complies with the following rules
        https://www.brainbashers.com/skyscrapershelp.asp
        Check also uniqueness and range of values
 -}
 
main :: IO ()
main = do
    putStr "start\n"
    let counts=6
    let iterations=[1..counts+2]
    let maxval=9
    let values=[1..maxval]
    let file = "skyscrapers_games/skyscrapers-"++show counts++"x"++show counts++".txt"
    lines <- openfile file iterations
    
    mapM_ print lines
    
    
    let linestmp= transpose lines
    {- PER AVERE LE RIGHE ESTRAGGO LA PRIMA LISTA E POI L'ULTIMA -}
    let constraintrow = zip ( init $ tail $ head lines) (init $ tail $ last lines)
    {- PER AVERE LE COLONNE DOPO AVER FATTO LA TRASPOSTA ESTRAGGO LA PRIMA LISTA E POI L'ULTIMA -}
    let constraintcol = zip (init $ tail $ head linestmp) (init$ tail$ last linestmp)
    {- RIMUOVO I BORDI -}
    let matrix= transpose $ init $ tail $ transpose $ init $ tail lines
    putStr "MATRICE EFFETTIVA\n"
    mapM_ print matrix
    if  False `elem` map checkduplicate matrix then putStr "duplicate in rows\n" else putStr "unique in rows\n"
    
    let matrixT= transpose matrix
    if  False `elem` map checkduplicate matrixT then print "duplicate in cols\n" else putStr "unique in cols\n"
    checkconstr(zip matrixT constraintrow)
    checkconstr(zip matrix constraintcol)
    putStr "end\n"
    
openfile :: Traversable t => FilePath -> t a -> IO (t [Int])
openfile string iterations= do
    handle <- openFile string ReadMode
    lines <- mapM(\x -> do
            contents <- hGetLine handle
            let a= map read (words contents)::[Int]
            return a) iterations
    hClose handle
    return lines
checkconstr :: Foldable t => t ([Int], (Int, Int)) -> IO ()
checkconstr var= do
    mapM_(\line -> do
        print line
        let row1= fst (snd line)
        let row2= snd (snd line)
        let aaaa=maxChange (fst line)
        let bbbb=maxChange (reverse (fst line))
        if row1/=0 && aaaa/=row1 then putStr "error\n" else putStr "ok\n"
        if row2/=0 && bbbb/=row2 then putStr "error\n" else putStr "ok\n"
        print aaaa
        print bbbb) var

checkduplicate :: (Ord a, Num a) => [a] -> Bool
checkduplicate xs = length xs == length (nub xs) && length xs == length (filter (<= 9) xs )

maxChange:: [Int] -> Int
maxChange xs = fst (foldl (\(counter,max) x -> if x>max then (counter+1,x) else (counter,max)) (0,0) xs)