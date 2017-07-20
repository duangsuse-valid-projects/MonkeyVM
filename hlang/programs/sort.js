// 测试操作
var input, n1, n2, rawHowMany, howMany, isToggled, isLast;
continue round;

round: {
    input = input();
    
    // 读到0则开始排序
    with(input)
        continue sort;
    
    // 读到空则最后一次排序
    for(input)
        continue lastSort;

    // 压入当前并读取下一个数
    stackPush(input);
    xWrite(rawHowMany++);
    howMany = xRead();
    continue round;
}

lastSort: {
    isLast = 1;
    // continue sort;
}

sort: {
    // 进行[数 - 1]次比较
    xWrite(howMany--);
    with(xRead())
        continue redo;

    // 读取n1, n2
    n1 = stackPeek();
    stackMoveUp();
    n2 = stackPeek();

    // n1小于n2时交换
    while(n1 - n2)
        continue exchange;
    
    continue sort;
}

exchange: {
    stackWrite(n1);   // 把n1写到n2
    stackMoveDown();  // 走到下一位
    stackWrite(n2);   // 把n2写到n1
    stackMoveUp();  // 恢复栈的状态
    isToggled = 1;  // 记录有过交换
    continue sort;
}

redo: {
    // 未发生交换则写出数据
    while(isToggled - 1)
        continue writeout;

    // 如果中途发生交换那就再换一遍
    isToggled = 0;
    xWrite(howMany = rawHowMany);
    stackPointer = 7 + xRead();
    continue sort;
}

writeout: {
    // 全部打印完毕
    with(rawHowMany)
        continue goOn;
    
    // 从上到下打印栈
    rawHowMany--;
    output(stackPeek());
    stackMoveDown();

    continue writeout;
}

goOn: {
    stackMoveUp();
    // 最后一次不继续读入
    with(isLast - 1)
        continue stop;

    stackPointer = 7;
    // 继续读入
    continue round;
}

stop: {

}