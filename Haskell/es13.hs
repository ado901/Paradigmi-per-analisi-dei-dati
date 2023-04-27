{- ach of two players (attacker A and defender D) rolls three dice
Dice are compared in couples

    A's highest die against D's highest die... and so on
    In case of equal values, D's die wins

Repeat the game N times and create a list of N integer results

    A wins with 3 dice... or 2, 1, 0

Optionally, also count the occurrences of each possible result (0-3) -}
import Data.Word (Word32)
import Data.Bits (xor, shiftL, shiftR)
import Data.Time.Clock.POSIX (getPOSIXTime)
import Data.List

type Rng32 = Word32

xorshift32 :: Rng32 -> Rng32
xorshift32 a = d where
  b = a `xor` (a `shiftL` 13)
  c = b `xor` (b `shiftR` 17)
  d = c `xor` (c `shiftL` 5)

randint :: (Int, Int) -> Rng32 -> (Int, Rng32)
randint (nmin, nmax) gen = (val, nxt) where
    nxt = xorshift32 gen
    val = nmin + (fromIntegral nxt) `mod` (nmax + 1 - nmin)

randints :: (Int, Int) -> Rng32 -> [Int]
randints range gen =
    val : randints range nxt
    where (val, nxt) = randint range gen

getRng32 :: IO Rng32
getRng32 = do
    now <- getPOSIXTime
    return (round (now * 1000) :: Rng32)

risikoDice:: [Int]
risikoDice= let dice1=take 3 $ randints(1,6) 21323 :: [Int] 
                dice2=take 3 $ randints(1,6) 123322 :: [Int]
            in zipWith (\x y -> if x>y then 1 else 0) (sort dice1) (sort dice2)
            