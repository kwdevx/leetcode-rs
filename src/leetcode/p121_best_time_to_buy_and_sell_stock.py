from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        # C1: max profit -> max(res,(D[Sell] - D[Buy]))
        # C2: no profit -> return 0
        # To acheive profit, we are looking for upward slope
        # How do we exhaust all possibility?

        # Solution 1: double for loop -> O(n^2)
        # Solution 2: sliding window
            # when do we move the left and right pointers?
            # we want lowest buy price -> have lower price, move the that point
            # we want highest buy price -> have higher price, move the that point

        res = 0

        if len(prices)<=1:
            return res

        # assume buying in the 0th day and sell in the 1th day
        l,r = 0,1

        while l<r:
            profit = prices[r] - prices[l]
            res = max(res,profit)
            if profit<0:
                l = r
                if r != len(prices)-1:
                    r+=1
            else:
                if r != len(prices)-1:
                    r+=1
                else:
                    l+=1


        # return 0 if no profit
        return res