'use strict'

const acorn = require('acorn/dist/acorn_loose');
const fs = require('fs');

const program = fs.readFileSync('./program.js');
const body = acorn.parse_dammit(program).body;

const buffer = [];

let globalConfig = {
    init: true,
    stack: true
};

// 键: 标识符名, 值: 对应内存位置
let idMap = {
    stackPointer: 0,
    variableNum: 1022,
    swap: 1023
};
// 键: 标识符名, 值: 对应程序位置
let tagMap = {
    end: 0
};

let memStart = 0;
function allocMem() {
    memStart++;
    return memStart;
}

let tagStart = 0;
function allocTag() {
    tagStart++;
    return tagStart;
}

function handleVarDeclaration(element) {
    let variables = 0;
    switch (element.kind) {
        case 'var':
            if(globalConfig.init) {
                buffer.push(':eyes: 0');
            }
            element.declarations.forEach((declarator) => {
                let position = allocMem();
                idMap[declarator.id.name] = position;
                if(globalConfig.init) {
                    buffer.push(`:memo::point_right:${position}`);
                }
                variables++;
            });

            if(globalConfig.stack) {
                // 初始化栈指针
                buffer.push(`:eyes:${idMap.stackPointer + variables}`);
                buffer.push(`:memo::point_right:${idMap.stackPointer}`); 
                // 写参数数量
                buffer.push(`:eyes:${variables}`);
                buffer.push(`:memo::point_right:${idMap.variableNum}`); 
            }
            break;
        default:
            throw new SyntaxError(`Unsupported declaration kind ${expression.kind}`);
    }
    
}

function handleLabeledStatement(element) {
    let startPositon = allocTag();
    tagMap[element.label.name] = startPositon;

    buffer.push(`:point_right:${startPositon}`);

    let statements = element.body.body.forEach((element) => {
        walk(element);
    })
}

// 赋值语句左边为返回值
function handleAssignmentExpression(expression) {
    let operator = expression.operator;
    let id = '';
    switch (operator) {
        case '+=':
            // 右边为内置函数/字面值/变量, 产生的'值'位于寄存器中
            walk(expression.right);

            // 左边为标识符
            id = expression.left.name;
            buffer.push(`:monkey_face::point_right:${idMap[id]}`); // 寄存器叠加本位值
            buffer.push(`:memo::point_right:${idMap[id]}`); // 将结果写入内存
            break;
        case '-=':
            walk(expression.right);
            buffer.push(`:memo::point_right:${idMap.swap}`);
            // 左边为标识符
            id = expression.left.name;
            buffer.push(`:eyes::point_right:${idMap[id]}`); // 将左边值读入寄存器
            buffer.push(`:see_no_evil::point_right:${idMap.swap}`); // 寄存器减去右边值
            buffer.push(`:memo::point_right:${idMap[id]}`); // 将结果写入内存
            break;
        case '=':
            walk(expression.right);

            id = expression.left.name;
            buffer.push(`:memo::point_right:${idMap[id]}`)
            break;
        default:
            throw new SyntaxError(`Unsupported operator ${operator}`);
    }
}

function handleCallExpr(expr) {
    let id = expr.callee.name;
    let argument = expr.arguments[0]; // 只接收1个参数
    embededFunctions(id ,argument);
}

function handleUpdateExpr(expr) {
    let id = expr.argument.name;
    if(expr.operator === '++') {
        buffer.push(`:thumbsup::point_right:${idMap[id]}`);
    } else {
        buffer.push(`:thumbsdown::point_right:${idMap[id]}`);
    }
}

// 有可选返回值!
// 所谓返回值: 被写进寄存器里的值!
function handleExprStatement(element) {
    let expression = element.expression;
    let id, argument;
    switch (expression.type) {
        case 'AssignmentExpression':
            handleAssignmentExpression(expression)
            break;
        case 'UpdateExpression':
            handleUpdateExpr(expression);
            break;
        case 'CallExpression':
            handleCallExpr(expression);
            break;
        case 'Literal':
            // 处理Notation
            if(expression.value === 'no stack') {
                globalConfig.stack = false;
            } else if (expression.value === 'no init') {
                globalConfig.init = false;
            } else {
                throw new SyntaxError(`Unsupported notation ${expression.value}`);
            }
            break;
        default:
            throw new SyntaxError(`Unsupported expression ${expression.type}`);
    }
}

function embededFunctions(id, param) {
    switch (id) {
        case 'problem':
            // 直接输出用以避开标签替换
            console.log(`//[${param.value}]`);
            break;
        case 'input':
            buffer.push(':poultry_leg:');
            break;
        case 'output':
            walk(param);
            buffer.push(':hankey:');
            break;
        case 'cast':
            walk(param);
            buffer.push(':loudspeaker:');
            break;
        case 'stackPush':
            // stackPush返回值为参数
            // 向下移动栈指针
            buffer.push(`:thumbsup::point_right:${idMap.stackPointer}`);
            // 读取参数到寄存器
            walk(param);
            // 将参数值写到栈指针所指向内存
            buffer.push(`:memo::point_right:${idMap.stackPointer}:point_right:`);
            break;
        case 'stackPop':
            // stackPop返回值为pop出来的值
            // 读取栈指针所指向内存到寄存器
            buffer.push(`:eyes::point_right:${idMap.stackPointer}:point_right:`);
            // 将寄存器数值写到swap
            buffer.push(`:memo::point_right:${idMap.swap}`);
            // 向上移动栈指针
            buffer.push(`:thumbsdown::point_right:${idMap.stackPointer}`);
            // 读取swap到寄存器
            buffer.push(`:eyes::point_right:${idMap.swap}`);
            break;
        case 'stackPeek':
            buffer.push(`:eyes::point_right:${idMap.stackPointer}:point_right:`);
            break;
        case 'stackWrite':
            walk(param);
            buffer.push(`:memo::point_right:${idMap.stackPointer}:point_right:`);
            break;
        case 'stackMoveUp':
            buffer.push(`:thumbsdown::point_right:${idMap.stackPointer}`);
            break;
        case 'stackMoveDown':
            buffer.push(`:thumbsup::point_right:${idMap.stackPointer}`);
            break;
        case 'xRead':
            // do nothing
            break;
        case 'xWrite':
            walk(param);
            break;
        case 'xWriteLiteral':
            buffer.push(`:eyes:${param.value}`);
            break;
        case 'xAddLiteral':
            buffer.push(`:monkey_face:${param.value}`);
            break;
        case 'xSubLiteral':
            buffer.push(`:see_no_evil:${param.value}`);
            break;
        case 'xAddMemory':
            buffer.push(`:monkey_face::point_right:${idMap[param.name]}`);
            break;
        case 'xSubMemory':
            buffer.push(`:see_no_evil::point_right:${idMap[param.name]}`);
            break;
        case 'xAddPointer':
            buffer.push(`:monkey_face::point_right:${idMap[param.name]}:point_right:`);
            break;
        case 'xSubPointer':
            buffer.push(`:see_no_evil::point_right:${idMap[param.name]}:point_right:`);
            break;
        default:
            throw new EvalError(`No such function ${id}`);
    }
}

// 有可选返回值!
// 所谓返回值: 被写进寄存器里的值!
function walk(element) {
    let id, name, argument, expression;
    let anonymousStartTag, anonymousEndTag;
    switch (element.type) {
        case 'VariableDeclaration':
            handleVarDeclaration(element);
            break;
        case 'LabeledStatement':
            handleLabeledStatement(element);
            break;
        case 'ExpressionStatement':
            handleExprStatement(element);
            break;
        case 'Literal':
            buffer.push(`:eyes:${element.value}`);
            break;
        case 'Identifier':
            id = element.name;
            buffer.push(`:eyes::point_right:${idMap[id]}`);
            break;
        case 'ContinueStatement':
            name = element.label.name;
            buffer.push(`:monkey: [${name}]`);
            break;
        case 'BinaryExpression':
            if (element.operator === '+') {
                walk(element.right);
                buffer.push(`:memo::point_right:${idMap.swap}`);
                walk(element.left);
                buffer.push(`:monkey_face::point_right:${idMap.swap}`);
            } else if (element.operator === '-') {
                walk(element.right);
                buffer.push(`:memo::point_right:${idMap.swap}`);
                walk(element.left);
                buffer.push(`:see_no_evil::point_right:${idMap.swap}`);
            } else {
                throw new SyntaxError(`Unsupported binary operator ${element.operator}`);
            }
            break;
        case 'IfStatement':
            walk(element.test);
            name = element.consequent.label.name;
            buffer.push(`:question::banana::monkey: [${name}]`);
            break;
        case 'WithStatement':
            walk(element.object);
            name = element.body.label.name;
            buffer.push(`:question::ghost::monkey: [${name}]`);
            break;
        case 'WhileStatement':
            walk(element.test);
            name = element.body.label.name;
            buffer.push(`:question::scream::monkey: [${name}]`);
            break;
        case 'ForStatement':
            walk(element.init);
            name = element.body.label.name;
            buffer.push(`:question::mailbox_with_no_mail::monkey: [${name}]`);
            break;
        case 'BreakStatement':
            buffer.push(`:monkey:${tagMap.end}`);
            break;
        case 'CallExpression':
            handleCallExpr(element);
            break;
        case 'AssignmentExpression':
            handleAssignmentExpression(element);
            break;
        case 'UpdateExpression':
            handleUpdateExpr(element);
            break;
        default:
            throw new SyntaxError(`Unsupported element ${element.type}`);
    }
}

body.forEach((element) => {
    walk(element);
});

// 输出
let tag = /\[(\w+?)\]/;
buffer.forEach((string) => {
    let result = tag.exec(string);
    if (result) {
        result = string.replace(result[0], tagMap[result[1]]);
        console.log(result);
    } else {
        console.log(string);
    }
})