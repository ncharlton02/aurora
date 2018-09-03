function max(num1, num2)

    if num1 > num2 then
       result = num1
    else
       result = num2
    end
 
    return result;
 end

bigger_num = max(5, 6)
print("Which is bigger, 5 or 6?", bigger_num)