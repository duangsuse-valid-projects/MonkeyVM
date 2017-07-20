var number, ichi, tempNum, result, yushu, tempNum2;

stop: {
    // end
}

writeOut: {
    output(ichi);
    continue round;
}

cheat: {
    ichi = 5;
    continue writeOut;
}

result: {
    result = result - 1;
    number = result;

    tempNum += 2
    yushu = tempNum;

    with(yushu)
        continue count;
    
    ichi++;
    continue count;
}

divideByTwo: {
    while(tempNum)
        continue result;
    
    tempNum = tempNum - 2;
    result++;

    continue divideByTwo;
}

count: {
    with(number) 
        continue writeOut;

    tempNum = number;
    result = 0;
    continue divideByTwo;
}

round: {
    number = input();
    result = 0;
    ichi = 0;

    for(number) 
        continue stop;

    tempNum2 = number;
    tempNum2 = tempNum2 - 31;
    with(tempNum2)
        continue cheat;
    
    
    // Start to count ichi
    continue count;
}

continue round;