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
int main(){
    greet("Tangle User");
}
```

---

You can also import blocks using macros inside the code block:

Define a variable:

```c config
const char* language = "C";
```
or even just a plain value:
```c a_value
42
```

Import and use the variable using macros:

```c variable_user use=[headers]
@[config]
printf("This program is written in %s.\n", language);
printf("and the meaning of life is %d\n", @[a_value]);
```

## custom executors

example haskell block. This evaluates a single haskell expression 
as long as the result is an instance of the `Show` typeclass
```haskell lambda
let x = (5::Int) in
  2 * x + 3
```

```haskell h_imports
import Data.IORef
```

example haskell block but using the haskell-io template, which requires
the code block to be of type `IO ()` to execute. Can be 
just the body of a `do` notation IO action
```haskell-io beta use=[h_imports]
let x = 5::Int
y <- newIORef x        -- int* y = 5;
putStrLn "new ref"
a <- readIORef y       -- int a = y*
putStrLn $ "current value: " ++ show a
putStrLn "increasing by 1"
writeIORef y (a+1)     -- y* = a + 1
a <- readIORef y       -- a = y*
putStrLn $ "new value: " ++ show a
```

Also c++

```c++ cpp_imports
#include <iostream>
```

```c++ cpp_example use=[cpp_imports]
std::cout << "Hello, World!" << std::endl;
```

