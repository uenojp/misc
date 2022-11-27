## NOTE

### スレッドのスケジューリング
ref. https://hpc-tutorials.llnl.gov/posix/creating_and_terminating/#thread-binding-and-scheduling

Linuxのtaskとして扱われるのでman schedを見た。
scheduling policyはなんとなく頭に入れておきたい。
例の本でLinux schedulerの歴史読んだけど忘れているので読み直してもいいかも。

### VLAの仕様
ref. https://ja.cppreference.com/w/c/language/array

配列のexpressoinが整数定数式でなくてもいいよ、というやつ。
`const size_t n = 10; char a[n];`したいよね。
C11からはサポートされない場合もあるので注意(`__STDC_NO_VLA__`が1のとき)。

### 配列からポインタへの暗黙変換
ref.
- https://ja.cppreference.com/w/c/language/conversion
- http://real-c.info/detailArray.html

VLA見ていたら目についた。
> 配列型のあらゆる左辺値式は、以下のいずれか
>   アドレス取得演算子の被演算子として
>   sizeof の被演算子として
>   配列初期化のために使用される文字列リテラルとして
> 以外の文脈で使用されたとき、その最初の要素を指す非左辺値ポインタに変換されます。
