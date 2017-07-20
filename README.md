# Monkey VM
这是一个普通~~怀旧~~,高效的`猴语言虚拟机`
coding已于活动结束以后将猴语言语法及实现移除,感谢mrkari的hlang项目使这个虚拟机不至于流产.

## 编译
```shell
git clone https://coding.net/duangsuse/MonkeyVM
cd MonkeyVM
cargo build
```

## 运行
```shell
cargo install
monkey_vm help
```
## 关于实现方法
MonkeyVM的实现方法(它是如何执行猴语言程序的?)与官方的解释器不同.
官方的部分实现方式可以看下面的错误输出:
```
[ParseError] Parse error on line 7: ...间用 0 做分隔符。给定的矩阵如下：1 2 3 45 6 7 81 4 ---------------------^ Expecting 'EOF', 'COMMENT', 'NEWLINE', 'IF', 'JUMP', 'LOG', 'INPUT', 'OUTPUT', 'READ', 'ADD', 'SUB', 'WRITE', 'INC', 'DEC', 'POINTER', got 'NUMBER'
```
MonkeyVM对于tags,采用了一个叫'TagManager'的实现.程序呼叫要求跳转到'行'(被解析成命令序号)时,从TagManager查询在程序解析时已经解析好的id-ln对应关系
对于Question(if),MonkeyVM会将其分成QNU,QPJ,QZJ,QNJ四种跳转情况,而不是另外开一条命令,,,>_>
MonkeyVM的目标是在执行程序上兼容官方的解释器,但不代表MonkeyVM下不会崩的程序官方解释器同样不会崩....
另外,问一下谁有官方解释器的源码啊???好像除了官方的和MonkeyVM没有其它的猴语言解释器了??

## 非常重要的东西
由于Rust的Lifetime,Borrow系统比较复杂,在遇到一个Borrowing的坑,尝试解决超过3小时以后,依然无法通过编译(如果有内行的话可以帮忙修一下...)
最终决定暂时拿Python3或其它的语言如C#重写一个解释器,原理与这个项目一致.这个项目的尾巴会在大约几个月,我大概学成Rust时再结,感谢希望尝试本项目的人滋瓷.
目前需要完成的基本只剩parser了,此时项目一共有745行Rust代码.
Python版本可以在Coding.net上搜索.
