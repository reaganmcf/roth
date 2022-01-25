"foo" "bar" eq if
"branch 1" print drop
else
"branch 2" print drop
end

15 10 > eq if
"15 > 10" print drop
else 15 5 eq if
"15 > 5" print drop
else
"none" print drop
end

IF Statement

1. Skip tokens until the else or end, whichever first

---

funcs

const MAX_SIZE 10 end
// const is just syntax sugar for the following
decl[0] CONST_X begin 5 end

// Cons: have no way to tell if stack has enough items before the function call!
decl add begin

- end

1 2 add --> 1 2 +

decl foo begin

end

Example: AOC21 #1

```
199 200 208 210 200 207 240 269 260 263
// push these onto the stack and add the variable we will use to track

decl did_inc begin
  // 208 210 200 207 240 269 260 263 199 200
  rot rot < if
    1 // add 1 to the variable at the top of the stack
  else
    0
  end
end

while dup null != begin
  mem cnt 0 // cnt is int with value 0

  did_inc cnt store
  // 208 210 200 207 240 269 263 1 (Op::MemRef { inner: Op })
end
```
