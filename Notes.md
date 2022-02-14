# Pointers (box)

`box <ident> <type> end`

Creates a pointer that can be referenced by <ident> of length

```
box myValue type::int end

myValue spill -> spill the value inside the box onto the stack
myValue store -> store the value on the top of the stack into the box
```

---

# Types

"1" type -> str
3 type -> number
true type -> bool
