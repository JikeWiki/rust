# 使用Box<T>来指向Heap上的数据

## 一、概述

`Box<T>`是一种最简单的智能指针，它实现了Deref trait和Drop trait。它允许你在heap上存储数据（而不是stack），Box在stack上有一个指针，指向heap上的真正数据，如下图

[49-01](./img/49-01.png)

出了把数据存储在heap上之外，`Box<T>`没有其他的性能开销了，同时也没有其他额外的功能。

## 二、Box<T>的常用场景

> p86 01:03
