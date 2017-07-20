// 阶乘
// 编译器版本: 1.2.0

var input, number, factor, result, remain;
//         乘数    乘数2

// give factor
round: {
    for(factor = number = input())
        continue stop;

    // continue calculate;
}

// use input, give number, factor
calculate: {
    xWrite(factor--);
    while(xRead())
        continue writeout;

    remain = xRead();
    result = 0;

    // continue mul;
}

// use number, factor, temp, give number
mul: {
    result += number;
    xWrite(remain--);

    if(xRead())
        continue mul;

    number = result;
    continue calculate;
}

writeout: {
    output(number);
    continue round;
}

stop: {
    // end
}