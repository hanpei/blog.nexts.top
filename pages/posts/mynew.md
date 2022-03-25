---
title: 实现MyNew方法
date: 2022-03-25
tags:
  - js
---

# 实现 MyNew 方法

今天摸鱼看到了这个 repo:  
[https://github.com/BetaSu/fe-hunter/issues/15](https://github.com/BetaSu/fe-hunter/issues/15)

> **要实现的功能**  
> 我们可以使用 new 实例化一个构造函数，请根据实例化过程中构造函数内部工作流程，实现类似功能的 MyNew 方法。

```js
function MyNew(fn, ...args) {
  // 实现...
}

function Person(name, age) {
  this.name = name;
  this.age = age;
}
const kasong = MyNew(Person, 'KaSong', 18);
console.log(kasong.age); // 18

function Something(name) {
  this.name = name;
  return { name: 'something' };
}
const something = MyNew(Something, 'XiaoMing');
console.log(something.name); // something
```

## 思路

完全忘记了 new 要干啥，只能面向 google 编程:

google search: mdn new。

> `https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/new`
>
> ### 描述
>
> new 关键字会进行如下的操作：
>
> 1. 创建一个空的简单 JavaScript 对象（即{}）；
> 2. 为步骤 1 新创建的对象添加属性**proto**，将该属性链接至构造函数的原型对象 ；
> 3. 将步骤 1 新创建的对象作为 this 的上下文 ；
> 4. 如果该函数没有返回对象，则返回 this。

开 vscode，复制上面步骤当成注释， 接下来就是 **copilot** 表演了 ☕️

```js
function MyNew(fn, ...args) {
  //1
  const obj = Object.create(null);
  //2
  Object.setPrototypeOf(obj, fn.prototype);
  //3
  const ret = fn.apply(obj, args);
  //4
  return ret instanceof Object ? ret : obj;
}
```

<br />
<br />

_纪念第一篇用自己的 🦀 [ONEPAGE](https://github.com/hanpei/onepage) 项目来构建 blog 了_
