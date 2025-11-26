# Comprehensions 扩展

Comprehensions 引入了对双变量推导式的支持。

双变量形式的推导式看起来与单变量的对应物相似。在可能的情况下，使用了相同的宏名称并添加了额外的宏签名。双变量推导式显著的区别在于，为列表和映射类型引入了 `transformList`、`transformMap` 和 `transformMapEntry` 支持，而不是更传统的 `map` 和 `filter` 宏。

- [Comprehensions 扩展](#comprehensions-扩展)
  - [All](#all)
  - [Exists](#exists)
  - [ExistsOne](#existsone)
  - [TransformList](#transformlist)
  - [TransformMap](#transformmap)
  - [TransformMapEntry](#transformmapentry)


## All

`all` 推导式测试列表或映射中的所有元素是否都满足给定的谓词。`all` 宏的求值方式与逻辑与（AND）一致，当遇到 `false` 值时会短路。

```cel
<list>.all(indexVar, valueVar, <predicate>) -> bool
<map>.all(keyVar, valueVar, <predicate>) -> bool
```

**示例:**

```cel
[1, 2, 3].all(i, j, i < j) // 返回 true
{'hello': 'world', 'taco': 'taco'}.all(k, v, k != v) // 返回 false

// 结合双变量推导式和单变量推导式
{'h': ['hello', 'hi'], 'j': ['joke', 'jog']}
    .all(k, vals, vals.all(v, v.startsWith(k))) // 返回 true
```

## Exists

`exists` 推导式测试列表或映射中是否存在任何满足给定谓词的元素。`exists` 宏的求值方式与逻辑或（OR）一致，当遇到 `true` 值时会短路。

```cel
<list>.exists(indexVar, valueVar, <predicate>) -> bool
<map>.exists(keyVar, valueVar, <predicate>) -> bool
```

**示例:**

```cel
{'greeting': 'hello', 'farewell': 'goodbye'}
    .exists(k, v, k.startsWith('good') || v.endsWith('bye')) // 返回 true
[1, 2, 4, 8, 16].exists(i, v, v == 1024 && i == 10) // 返回 false
```

## ExistsOne

`exists_one` 推导式测试列表或映射中是否只存在一个满足给定谓词表达式的元素。为了与单变量的 `exists_one` 宏语义保持一致，此推导式不会短路。

```cel
<list>.existsOne(indexVar, valueVar, <predicate>)
<map>.existsOne(keyVar, valueVar, <predicate>)
```

此宏也可以使用 `exists_one` 函数名，以兼容同名的单变量宏。

**示例:**

```cel
[1, 2, 1, 3, 1, 4].existsOne(i, v, i == 1 || v == 1) // 返回 false
[1, 1, 2, 2, 3, 3].existsOne(i, v, i == 2 && v == 2) // 返回 true
{'i': 0, 'j': 1, 'k': 2}.existsOne(i, v, i == 'l' || v == 1) // 返回 true
```

## TransformList

`transformList` 推导式将映射或列表转换为列表值。推导式的输出表达式决定了输出列表的内容。列表中的元素可以根据谓词表达式进行可选的筛选，其中满足谓词的元素将被转换。

```cel
<list>.transformList(indexVar, valueVar, <transform>)
<list>.transformList(indexVar, valueVar, <filter>, <transform>)
<map>.transformList(keyVar, valueVar, <transform>)
<map>.transformList(keyVar, valueVar, <filter>, <transform>)
```

**示例:**

```cel
[1, 2, 3].transformList(indexVar, valueVar,
  (indexVar * valueVar) + valueVar) // 返回 [1, 4, 9]
[1, 2, 3].transformList(indexVar, valueVar, indexVar % 2 == 0,
  (indexVar * valueVar) + valueVar) // 返回 [1, 9]
{'greeting': 'hello', 'farewell': 'goodbye'}
  .transformList(k, _, k) // 返回 ['greeting', 'farewell']
{'greeting': 'hello', 'farewell': 'goodbye'}
  .transformList(_, v, v) // 返回 ['hello', 'goodbye']
```

## TransformMap

`transformMap` 推导式将映射或列表转换为映射值。推导式的输出表达式决定了输出映射条目的值；但是，键保持不变。映射中的元素可以根据谓词表达式进行可选的筛选，其中满足谓词的元素将被转换。

```cel
<list>.transformMap(indexVar, valueVar, <transform>)
<list>.transformMap(indexVar, valueVar, <filter>, <transform>)
<map>.transformMap(keyVar, valueVar, <transform>)
<map>.transformMap(keyVar, valueVar, <filter>, <transform>)
```

**示例:**

```cel
[1, 2, 3].transformMap(indexVar, valueVar,
  (indexVar * valueVar) + valueVar) // 返回 {0: 1, 1: 4, 2: 9}
[1, 2, 3].transformMap(indexVar, valueVar, indexVar % 2 == 0,
  (indexVar * valueVar) + valueVar) // 返回 {0: 1, 2: 9}
{'greeting': 'hello'}.transformMap(k, v, v + '!') // 返回 {'greeting': 'hello!'}
```

## TransformMapEntry

`transformMapEntry` 推导式将映射或列表转换为映射值；但是，此转换期望条目表达式是映射字面量。如果转换产生的条目在目标映射中复制了键，则推导式将出错。请注意，键的相等性是使用 CEL 相等性确定的，它断言相等的数值（即使它们类型不同）也会导致键冲突。

映射中的元素可以根据谓词表达式进行可选的筛选，其中满足谓词的元素将被转换。

```cel
<list>.transformMapEntry(indexVar, valueVar, <transform>)
<list>.transformMapEntry(indexVar, valueVar, <filter>, <transform>)
<map>.transformMapEntry(keyVar, valueVar, <transform>)
<map>.transformMapEntry(keyVar, valueVar, <filter>, <transform>)
```

**示例:**

```cel
// 返回 {'hello': 'greeting'}
{'greeting': 'hello'}.transformMapEntry(keyVar, valueVar, {valueVar: keyVar})
// 反向查找，要求列表中的所有值都是唯一的
[1, 2, 3].transformMapEntry(indexVar, valueVar, {valueVar: indexVar})

{'greeting': 'aloha', 'farewell': 'aloha'}
  .transformMapEntry(keyVar, valueVar, {valueVar: keyVar}) // 错误，重复的键
```
