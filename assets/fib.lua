times = 50
count = 1
result = 0

function fib(x, y)
    local z = x + y
    y = x
    count = count + 1

    if count < times then
        fib(z, y)
    else
        result = x
    end
end

fib(1, 1)
print(result)