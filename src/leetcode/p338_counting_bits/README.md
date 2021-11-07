# p338_counting_bits

1. Method 1: Dynamic Programming
   
   `i` run from `0` to `num`. So when calculating results[i], we have known results[i >> 1]. The difficult of number of 1's is only decided by last bit of results[i].
   
   ```rust
   results[i] = results[i >> 1] + (i as i32 & 1);
   ```

2. Method 2: call function `i32.count_ones()`

---

1. 方法1：动态规划

    `i`从`0`运行到`num`。所以计算results[i]时，我们已经知道results[i >> 1]。1's的个数差只由results[i]最后一个比特决定。
   
   ```rust
   results[i] = results[i >> 1] + (i as i32 & 1);
   ```

2. 方法2：调用函数`i32.count_ones()`
