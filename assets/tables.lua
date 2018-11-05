local numbers = {} -- Create a tables

-- Tables can have variables
numbers.pi = 3.1415
numbers.one_fifth = 1 / 5

num = numbers

print("Pi is " .. num.pi)
print("1/5 is " .. num.one_fifth)


-- tables can have functions!
local std_math = {}

function std_math.double(x)
    return x * 2
end

local math = std_math

local ten = math.double(5);
print("5 * 2 = "  .. ten)

print("")
print("Numbers: " .. numbers)
print("Double Function: " .. math.double)
