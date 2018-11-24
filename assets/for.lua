require("lib/core")

x = 0
for i=0, 10, 1 do
    x = x + 5
    print(i)
end

assert(x, 50)