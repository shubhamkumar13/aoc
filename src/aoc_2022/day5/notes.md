## HACK ALERT!!

Adding a "duumy" string literal manually on the first line of input validates spaces
and makes the following input possible

```
[
    "dummy\n    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ", 
    "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2"
]
```

and without the "dummy" string literal I would never guess the position of `[D]` 
because the `split` method completely ignores anything before the first no space character

```
[
    "[D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ", 
    "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2"
]

But this is not the right way to solve this problem in the wild because this assumes control over the input.

Unfortunately getting the answer is ok for AOC but this feels like cheating.