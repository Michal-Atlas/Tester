# **M**ichal **A**tlas **Te**ster

Usage:

```
mate [Directory containing test] [Command to pass to]
```

This program takes a directory of the form:

```
tests
├── test_1
│   ├── exit
│   ├── in
│   └── out
└── test_n
    ├── exit
    ├── in
    └── out
```

The program then runs each subdirectory as a separate test,
passing `in` on `stdin` to the program.
And then compares the output to the `out`,
and the exit code to `exit`, reporting the findings.

