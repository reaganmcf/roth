# roth

A concatenative programming language, designed to be simple and easy to implement.

## Concatenative what?

A concatenative programming language, also known as a stack based programming language, is a language that relies on a stack machine model for passing parameters.

This sounds more confusing than it actually is - so here's an example of an the `+` operation in `roth` to explain it with code.

If we want to add 2 numbers together, let's say `5` and `10`, then we write the numbers **first**, followed by the `+` operator - like so

```js
5 10 +
```

This will get evaluated to 15, since the runtime will read the `+` operator, and pop 2 elements off the stack, and push the result of adding them together. Pretty neat!

### Features

##### Simple arithmetic

```js
1 2 + 5 *
```
