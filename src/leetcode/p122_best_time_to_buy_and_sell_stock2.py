from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        res = 0

        if len(prices) <2:
            return res

        # keep buying low and selling high
        # i = right pointer to detect low or high

        for i in range(1,len(prices)):
            if prices[i] - prices[i-1] > 0:
                res+=prices[i] - prices[i-1]

        return res