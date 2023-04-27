import System.Random
import Control.Monad (when)
askUser num= do
    putStr "What's your guess? "
    guess <- getLine
    if guess==show num
        then return 1
    else do 
        if guess>show num
            then return 0
            else return (-1)
    

        
increment :: Int -> Int
increment x = x + 1
main= do
    gen <- getStdGen
    let (secret, newGen) = randomR (1,90) gen :: (Int, StdGen)
    putStrLn $ "I'm thinking of a number between 1 and 90.  Can you guess it?"
    print secret
    guess <- askUser secret
    let count=0
    when (guess /= 1) $ do
        let count= count+1
        if guess==0
            then putStrLn "Too high!"
            else putStrLn "Too low!"
        
        

        

    

    