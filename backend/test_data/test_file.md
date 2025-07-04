### Test file

This test defines a custom function in one block and uses it in the main block by importing it.

Define headers:

```c headers
#include <stdio.h>
```

Define a helper function in its own block:

```c helper
void greet(const char* name) {
    printf("Hello, %s!\n", name);
}
```
execute a python block:
```python monty
a = 'SPAM'
for i in range(4):
    print(a)
```

Define main block `run`:

```c use=[headers,helper] main_block
    greet("Tangle User");
```
