var money, full, empty, cap, total;

stop: {
    // 结束
}

// 即, 买一瓶酒继续喝
repower: {
    full++;
    continue drink;
}

exchange: {
    // 用空瓶子换
    empty = empty - 2
    if(empty)
        continue repower;
    empty += 2;

    // 用盖子换
    cap = cap - 4
    if(cap)
        continue repower;
    cap += 4;

    output(total);
    continue round;
}

drink: {
    // 没酒喝, 去换
    with(full)
        continue exchange;

    // 喝下一瓶
    full = full - 1;
    total++;
    empty++;
    cap++;

    // 还有, 继续喝
    continue drink;
}

buy: {
    // 花钱买酒
    money = money - 2;
    full = full + 1;
    if(money)
        continue buy;
    
    // 购买完毕
    full = full - 1;
    continue drink;
}

round: {
    // 初始化
    money = input();
    full = 0;
    empty = 0;
    cap = 0;
    total = 0;

    // 无输入则退出
    for(money) 
        continue end;
    // 进入购买环节
    continue buy;
}

continue round;