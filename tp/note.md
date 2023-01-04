# original == tp(tp(original))であってほしい

```original
a

b
```

```tp
a b
```

```tp|tp
a
␣
b
```

1回目のtpを入力としたときスペースを作らないようにすれば、tp|tpの余分なスペースは生まれない

しかし、次の2つを転置したときともに'a b'になる
```shell
diff <(echo -e 'a\n\nb' | cargo run -q) <(echo -e 'a\n \nb' | cargo run -q)
# $? is 0
```

```original
a

b
```

```original
a
␣
b
```

