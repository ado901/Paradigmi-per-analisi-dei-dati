doubleMe x = x + x

doubleUs x y = doubleMe x + doubleMe y

lucky :: (Integral a) => a -> String
lucky 7 = "LUCKY NUMBER SEVEN!"
lucky x = "Sorry, you're out of luck, pal!"