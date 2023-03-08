pyth= [(a,b,c) for a in (range(1,11)) for b in range(1,11) for c in range(1,11) if a**2+b**2==c**2 and a+b+c==24 ]
print(pyth)