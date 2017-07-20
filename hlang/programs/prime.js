// 编译器: 1.2.0

problem('prime_factorizer');
'no stack';
'no init';

var number, prime, lastSuccess, result;

// continue round;

round: {
    xWrite(number = input());

    for(xRead())
        continue stop;
    
    lastSuccess = xRead();
    prime = 2;
    result = 0;
    // continue mod;
}

mod: {
    // 反复将number减去prime, 直到number为0或负
    result++;
    xWrite(number -= prime);
    if(xRead())
        continue mod;
    
    // 整除, 写出质数
    with(xRead())
        continue success;

    // 无法整除, 恢复Number, 质数+1继续取mod
    result = 0;
    number = lastSuccess;
    prime++;
    continue mod;
}

success: {
    output(prime);

    // 结果等于1表示执行完毕
    xWrite(result--);
    with(xRead())
        continue round;

    // 继续取mod
    xAddLiteral(1);
    lastSuccess = number = xRead();
    result = 0;
    continue mod;
}

stop: {

}