type Name = String
type PhoneNumber = String
type PhoneBook = [(Name,PhoneNumber)]
getPhoneBook:: Name -> PhoneBook -> Maybe PhoneNumber
getPhoneBook name [] = Nothing
getPhoneBook name ((k,v):xs)= if name==k then Just v else getPhoneBook name xs

data ARENA_SIZE = ARENA_SIZE Float Float deriving (Show)
data BouncingBall= BouncingBall {x::Float,y::Float,dx::Float,dy::Float, size::Float} deriving (Show)
move :: BouncingBall -> BouncingBall
move (BouncingBall x y dx dy size)= BouncingBall (x+dx) (y+dy) dx dy size