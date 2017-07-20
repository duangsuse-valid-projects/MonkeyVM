var input, wei, width, realNum, result;

continue round;

// use -, give width, realNum, input
round: {
    input = input();

    for(input)
        continue stop;

    // 100到999一个算法(输入189~2889)
    input = input - 9;
    width = 2;
    realNum = 10;
    while (input - 189)
        continue getNumber;

    // 9到99一个算法(输入9~189)
    input = input - 189;
    width = 3;
    realNum = 100;
    // continue getNumber;
}

// use input, realNum, width, give width, readNum
getNumber: {
    realNum++;
    input = input - width;
    if (input)
        continue getNumber;
    
    realNum--; // realNum: 真实的数字
    input += width;

    wei = width + 1 - before(); // b: input
    
    width = 100;
    with(wei - 3)
        continue writeWei;

    width = 10;
    with(wei - 2)
        continue writeWei;

    width = 1;
    // continue writeWei;
}

// use width, realNum, give temp
writeWei: {
    realNum = realNum - 10;
    result++;
    if (realNum)
        continue writeWei;
    
    wei--;
    with(wei)
        continue writeOut;

    realNum = result + 1;
    continue writeWei;
}

writeOut: {
    output(realNum + 10);
    continue round;
}

stop: {
    // end
}