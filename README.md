#converting md files into HTML tags in console
===

```cargo run -- file.md -w --css file.css -e```

#Output
======

```    Finished dev [unoptimized + debuginfo] target(s) in 0.74s
     Running `target/debug/mdrend file.md -w --css file.css -e`
Input : "file.md"

Start(Heading(1))
Text(Borrowed("hello"))
End(Heading(1))
Start(Paragraph)
Text(Borrowed("welcome "))
Start(Emphasis)
Text(Borrowed("magdi"))
End(Emphasis)
End(Paragraph)

<!DOCTYPE html><html><head><meta charset="utf-8"><link rel="stylesheet" type="text/css" href="file.css"></link></head><body><h1>hello</h1>
<p>welcome <em>magdi</em></p>
</body></html>
Done!
```