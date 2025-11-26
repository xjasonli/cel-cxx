# CEL 参考文档

本目录包含 cel-cxx 中实现的通用表达式语言（CEL）的完整参考文档。

## 核心文档

- [基本概念和语法](01-basic-concepts.md) - 语言基础、语法、类型和运算符
- [标准库](02-standard-library.md) - 内置函数和标准操作
- [可选值](03-optional.md) - 可选类型处理和安全导航
- [扩展](04-extensions.md) - 扩展库和高级特性

## 扩展库

以下扩展可用，文档位于 [extensions](extensions/) 目录：

- [绑定扩展](extensions/bindings.md) - 变量绑定和作用域增强
- [推导式扩展](extensions/comprehensions.md) - 列表和映射推导式
- [编码器扩展](extensions/encoders.md) - Base64 和 URL 编码/解码
- [列表扩展](extensions/lists.md) - 增强的列表操作
- [数学扩展](extensions/math.md) - 数学函数和操作
- [协议缓冲区扩展](extensions/proto.md) - Protobuf 消息支持
- [正则表达式扩展](extensions/re.md) - 基本正则表达式操作
- [正则扩展](extensions/regex.md) - 高级正则表达式支持
- [集合扩展](extensions/sets.md) - 集合操作和工具
- [字符串扩展](extensions/strings.md) - 高级字符串操作

## 快速导航

- **快速开始**: 从[基本概念](01-basic-concepts.md)开始了解 CEL 语法和类型
- **常用操作**: 查看[标准库](02-standard-library.md)了解内置函数
- **高级特性**: 探索[扩展](04-extensions.md)获取额外功能

## 相关文档

- [主文档](../README.md) - 完整的 cel-cxx 文档
- [函数注册](../function-registration.md) - 如何注册自定义函数
- [高级特性](../advanced-features.md) - 高级使用模式

