# p331_verify_preorder_serialization_of_a_binary_tree

The out degree of the tree is equal to the in degree. So we can check whether out degree minus in degree equals 0.

We should make sure that anytime out degree isn't less than in degree. Out degree is the number of positions where nodes can be placed. In degree is the number of positions where nodes have been placed. When in degree is larger than out degree, it means that there are some nodes dangling, which is illegal.

---

树的出度和入度应该相同，所以我们可以检查出度减入度是否为0。

我们需要保证任何时候出度都不小于入度。出度表示可以放置节点的位置个数，入度表示已经放置的节点个数。如果入度大于出度，意味着有空悬的节点，这是不合法的。
