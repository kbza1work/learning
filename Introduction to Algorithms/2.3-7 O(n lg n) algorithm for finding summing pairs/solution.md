```
Describe a O(n lg n) algorithm that, given a set S of n integers and another integer x, determines whether or not there exist two elements in S whose sum is exactly x.
```

Solution 1
--

1. Create an empty auxiliary array T [O(1)]
2. For each element y in S:
   - Let d =   x - y [O(1)]
   - Search T for d using binary search [O(lg n)]
   - If d isn't found in T, then insert y into T using binary search to insert it such that T remains sorted. [O(lg n)] If d is found in T then there must exist another element that sums with y to give x so return immediately.

Total time for worst-case is O(1) + n * (O(1) + O(lg n) + O(lg n)) = O(1) + O(n) + 2 O(n lg n) = O(n lg n)


Solution 2 from (@domgetter)
--

1. Sort S. [O(n lg n)]
2. For each element in S:
  - , Do a binary search to find the difference between that element and x [O(n)]

Total time for worst-case is O(lg n) \* O(n) = O(n lg n)
