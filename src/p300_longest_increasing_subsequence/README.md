# p300_longest_increasing_subsequence
1. Method 1: Dynamic programming
    
    `dp[i]` indicates the length of the longest substring ending with the i-th element, which is equal to the length of the longest string with the last number smaller than the i-th element plus one. Therefore, for each element, we traverse the previous dp value, and the time complexity is O(N^2).

2. Method 2: Greedy + Dichotomy (optimized from Method 1)

    In Method 1, because we don't know which element should be followed by the i-th element, it took a lot of time to find the length of the longest substring ending with the i-th element.
    
    Suppose, before the i-th element, the length of the longest substring is x. By comparing the last element of this substring with the i-th element, we can know whether the i-th element can be followed to form a string of length x+1.

    If we group all substrings before the i-th element by length. When we find that the i-th element cannot form a string of length x+1, we will compare with the end elements of all substrings of length x-1 to see if a string of length x can be formed. In this way, we can find the length of the longest substring ending with the i-th element, but the time complexity is still O(N), and then we optimize it.

    Because the elements in the substring are increasing, it is easy to know that when there are multiple substrings with the same length, the smallest element at the end is most likely to have the i-th element following it. We only need to compare the i-th element with the smallest last element in each group of substrings of the same length.

    Use an array `array` to record what is the smallest end element in each group of substrings of the same length. This way we can skip a set of strings through once comparison.

    At the same time, we know that the substring is increasing, which means that for a substring of length j, the (j-1)-th character must be less than the j-th character. So `array[j-1]` must be smaller than `array[j]`, that is, `array` is increasing. Then we can add a binary search to further shorten the time.

---

1.  方法1：动态规划
    
    `dp[i]`表示以第i个元素结尾的最长子串的长度，等于最后一个数比第i个元素小的最长字串长度加一。所以对每个元素，都遍历一遍先前的dp值，时间复杂度O(N^2)。

2.  方法2：贪心+二分（从方法1优化）

    方法1因为不知道第i个元素应该接在先前哪个元素后面，在找以第i个元素结尾的最长子串长度时，花了很多时间。
    
    假设，第i个元素前，最长的子串长度为x。用这条子串的最后一个元素和第i个元素比较，我们就能知道第i个元素能不能接在后面形成长度为x+1的串。

    如果我们把第i个元素前的所有子串按长度分组。当我们发现第i个元素不能形成x+1长度的串时，我们就去和所有长度为x-1的子串末尾元素对比，看能否形成长度为x的串。这样我们能找到以第i个元素结尾的最长子串的长度，但是时间复杂度仍是O(N)，接着我们优化它。

    因为子串内的元素是递增的，易知，当先前有多条子串长度相同时，末尾元素最小的，最有可能让第i个元素接在他后面。我们只需要让第i个元素，和每组相同长度的子串中，最小的末尾元素比较。

    用一个数组array去记录，每组长度相同的子串中，最小的末尾元素是多少。这样我们可以通过一次比较，跳过一组字符串。

    同时，我们知道子串内是递增的，这意味着对于长度为j的子串，第j-1个字符一定小于第j个字符。所以array[j-1]一定小于array[j]，也就是array数组是递增的。那么我可以再加上一个二分搜索去进一步缩短时间。
