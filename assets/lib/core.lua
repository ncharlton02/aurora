-- The aurora std library (must be manually imported)

-- Test if the library was loaded already. 
-- If loaded then fail!
if test_global_core_lib_loaded == false then
    fail("[Aurora Internal Error] Core library loaded twice!")
end
test_global_core_lib_loaded = true

function assert(x, y)
    if x == y then
        return 0;
    end

    fail("Assert failed! Expected " .. x .. " but found " .. y)
end