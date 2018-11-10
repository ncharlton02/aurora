-- The aurora std library (must be manually imported) -- 

function assert(x, y)
    if x == y then
        return 0;
    end

    fail("Assert failed! Expected " .. x .. " but found " .. y)
end