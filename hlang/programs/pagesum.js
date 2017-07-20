var total, pageNum;

stop: {
    // stop
}


writeout: {
    output(pageNum);
    continue round;
}

count: {
    with(total)
        continue writeout;

    pageNum++;
    total = total - pageNum;

    continue count;

}

round: {
    total = input();
    pageNum =  0;

    for(total)
        continue stop;

    continue count;
}

continue round; 