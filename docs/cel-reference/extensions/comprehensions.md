# Comprehensions Extension

Comprehensions introduces support for two-variable comprehensions.

- [Comprehensions Extension](#comprehensions-extension)
  - [All](#all)
  - [Exists](#exists)
  - [ExistsOne](#existsone)
  - [TransformList](#transformlist)
  - [TransformMap](#transformmap)
  - [TransformMapEntry](#transformmapentry)


The two-variable form of comprehensions looks similar to the one-variable counterparts. Where possible, the same macro names were used and additional macro signatures added. The notable distinction for two-variable comprehensions is the introduction of `transformList`, `transformMap`, and `transformMapEntry` support for list and map types rather than the more traditional `map` and `filter` macros.

## All

Comprehension which tests whether all elements in the list or map satisfy a given predicate. The `all` macro evaluates in a manner consistent with logical AND and will short-circuit when encountering a `false` value.

```cel
<list>.all(indexVar, valueVar, <predicate>) -> bool
<map>.all(keyVar, valueVar, <predicate>) -> bool
```

**Examples:**

```cel
[1, 2, 3].all(i, j, i < j) // returns true
{'hello': 'world', 'taco': 'taco'}.all(k, v, k != v) // returns false

// Combines two-variable comprehension with single variable
{'h': ['hello', 'hi'], 'j': ['joke', 'jog']}
    .all(k, vals, vals.all(v, v.startsWith(k))) // returns true
```

## Exists

Comprehension which tests whether any element in a list or map exists which satisfies a given predicate. The `exists` macro evaluates in a manner consistent with logical OR and will short-circuit when encountering a `true` value.

```cel
<list>.exists(indexVar, valueVar, <predicate>) -> bool
<map>.exists(keyVar, valueVar, <predicate>) -> bool
```

**Examples:**

```cel
{'greeting': 'hello', 'farewell': 'goodbye'}
    .exists(k, v, k.startsWith('good') || v.endsWith('bye')) // returns true
[1, 2, 4, 8, 16].exists(i, v, v == 1024 && i == 10) // returns false
```

## ExistsOne

Comprehension which tests whether exactly one element in a list or map exists which satisfies a given predicate expression. This comprehension does not short-circuit in keeping with the one-variable exists one macro semantics.

```cel
<list>.existsOne(indexVar, valueVar, <predicate>)
<map>.existsOne(keyVar, valueVar, <predicate>)
```

This macro may also be used with the `exists_one` function name, for compatibility with the one-variable macro of the same name.

**Examples:**

```cel
[1, 2, 1, 3, 1, 4].existsOne(i, v, i == 1 || v == 1) // returns false
[1, 1, 2, 2, 3, 3].existsOne(i, v, i == 2 && v == 2) // returns true
{'i': 0, 'j': 1, 'k': 2}.existsOne(i, v, i == 'l' || v == 1) // returns true
```

## TransformList

Comprehension which converts a map or a list into a list value. The output expression of the comprehension determines the contents of the output list. Elements in the list may optionally be filtered according to a predicate expression, where elements that satisfy the predicate are transformed.

```cel
<list>.transformList(indexVar, valueVar, <transform>)
<list>.transformList(indexVar, valueVar, <filter>, <transform>)
<map>.transformList(keyVar, valueVar, <transform>)
<map>.transformList(keyVar, valueVar, <filter>, <transform>)
```

**Examples:**

```cel
[1, 2, 3].transformList(indexVar, valueVar,
  (indexVar * valueVar) + valueVar) // returns [1, 4, 9]
[1, 2, 3].transformList(indexVar, valueVar, indexVar % 2 == 0,
  (indexVar * valueVar) + valueVar) // returns [1, 9]
{'greeting': 'hello', 'farewell': 'goodbye'}
  .transformList(k, _, k) // returns ['greeting', 'farewell']
{'greeting': 'hello', 'farewell': 'goodbye'}
  .transformList(_, v, v) // returns ['hello', 'goodbye']
```

## TransformMap

Comprehension which converts a map or a list into a map value. The output expression of the comprehension determines the value of the output map entry; however, the key remains fixed. Elements in the map may optionally be filtered according to a predicate expression, where elements that satisfy the predicate are transformed.

```cel
<list>.transformMap(indexVar, valueVar, <transform>)
<list>.transformMap(indexVar, valueVar, <filter>, <transform>)
<map>.transformMap(keyVar, valueVar, <transform>)
<map>.transformMap(keyVar, valueVar, <filter>, <transform>)
```

**Examples:**

```cel
[1, 2, 3].transformMap(indexVar, valueVar,
  (indexVar * valueVar) + valueVar) // returns {0: 1, 1: 4, 2: 9}
[1, 2, 3].transformMap(indexVar, valueVar, indexVar % 2 == 0,
  (indexVar * valueVar) + valueVar) // returns {0: 1, 2: 9}
{'greeting': 'hello'}.transformMap(k, v, v + '!') // returns {'greeting': 'hello!'}
```

## TransformMapEntry

Comprehension which converts a map or a list into a map value; however, this transform expects the entry expression be a map literal. If the tranform produces an entry which duplicates a key in the target map, the comprehension will error. Note, that key equality is determined using CEL equality which asserts that numeric values which are equal, even if they don't have the same type will cause a key collision.

Elements in the map may optionally be filtered according to a predicate expression, where elements that satisfy the predicate are transformed.

```cel
<list>.transformMapEntry(indexVar, valueVar, <transform>)
<list>.transformMapEntry(indexVar, valueVar, <filter>, <transform>)
<map>.transformMapEntry(keyVar, valueVar, <transform>)
<map>.transformMapEntry(keyVar, valueVar, <filter>, <transform>)
```

**Examples:**

```cel
// returns {'hello': 'greeting'}
{'greeting': 'hello'}.transformMapEntry(keyVar, valueVar, {valueVar: keyVar})
// reverse lookup, require all values in list be unique
[1, 2, 3].transformMapEntry(indexVar, valueVar, {valueVar: indexVar})

{'greeting': 'aloha', 'farewell': 'aloha'}
  .transformMapEntry(keyVar, valueVar, {valueVar: keyVar}) // error, duplicate key
```
