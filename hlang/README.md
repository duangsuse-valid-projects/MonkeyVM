#### About
这个仓库中保存为了参加[鸡年猴语言](https://coding.net/s/bb34f007-f1dc-4172-9ea9-6d2ed77292ef)活动而开发的编译器, 可以将一种精简的Javascript编译为:coding:语言, 此外仓库中还存放了用来解决谜题的程序(位于programs目录)和一些临时文件(all-program, program.js)
#### How to use it
此编译器基于Nodejs编写
```
// 编译本目录下program.js文件并将结果输出到控制台
$ node index.js
// 编译本目录下program.js文件并将结果输出到result文件
$ node index.js > result
```
可以将programs目录下的程序拷贝到上级目录, 重命名为program.js然后编译. 编译器有设计变更, factorial及以后谜题的程序文件可正常编译.