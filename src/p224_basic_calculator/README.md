# p224_basic_calculator
The difficulty of this question is about the addition and subtraction in the brackets.

We use a variable to store that we should add or subtraction the next number. The problem with this is that after encountering a pair of parentheses, the addition and subtraction inside the parentheses should not change the value outside the parentheses.

So we use a stack to store the variable to solve the problem. When reading ‘(’, we push the same value into the stack and pop the new value when reading the paired ‘)’. This makes sure that we can use the right value after leaving the parentheses.

---

这道题的难点是括号内的加减法。

我们用一个变量存储下一个数是加还是减。这里有个问题，当进入一个括号后，括号内的加减情况不应对记录括号外加减情况的变量产生影响。

要解决这个问题，我们用一个栈去存这个变量。当读到‘(’时，我们压入一个相同的值，读到‘)’时，把新的值弹出。这样确保我们能在离开括号后使用正确的值。