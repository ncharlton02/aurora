-- Calculates the first 50 fibonacci numbers
-- Note: To see a non-recursive version of this 
-- check the loops.lua file!
times = 50

function fib(x, y, count)
    local z = x + y
    y = x
    count = count + 1

    if count < times then
        return fib(z, y, count)
    else
        return x
    end
end

x = fib(1, 1, 1)
print(x)