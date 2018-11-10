require("lib/core")
num = 5 + 6
str = "This is a string"
boolean = true

print("Number", num)
print("String", str)
print("Boolean", boolean)

-- Numbers, strings, and booleans are primitives and are passed by value
x = 5
y = x 
y = 6

assert(x, 5)

-- Tables are passed by reference
x = {}
y = x
y.foo = 5
print("TEST")
assert(x.foo, 5)

