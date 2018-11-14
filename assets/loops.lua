-- Calculates the first 50 fibonacci numbers
-- Note: To see a recursive version of this 
-- check the fib.lua file!
require("lib/core")

function fib(count)
    local index = 1;
    local x = 1;
    local y = 0;
    local z = 0;

    while index < count do
        z = x
        x = x + y
        y = z
        index = index + 1

        print(index .. ": (" .. x .. ", " .. y .. ")")
    end

    return x;
end

local result = fib(50);
print(result)
assert(result, 12586269025)