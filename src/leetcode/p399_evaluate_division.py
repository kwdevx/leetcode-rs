import collections
from typing import List


class Solution:
    def calcEquation(self, equations: List[List[str]], values: List[float], queries: List[List[str]]) -> List[float]:
        hm = collections.defaultdict(dict)
        res = []
        for i, (a,b) in enumerate(equations):
            hm[a][b] = values[i]
            hm[b][a] = 1/values[i]


        #* bfs
        def bfs(start,target):
            acc = 1
            visited=set()
            q=collections.deque()
            q.append((1,start))

            while q:
                (acc, start) = q.popleft()
                if start in visited:
                    continue
                visited.add(start)

                for temp_to in hm[start]:
                    edge = hm[start][temp_to]
                    if temp_to == target:
                        return acc * edge
                    else:
                        q.append((acc*edge,temp_to))
            return -1


        for (start,target) in queries:
            res.append(bfs(start,target))

        # #* dfs
        # def dfs(visited,q_from,q_to,acc):
        #    if q_from in visited:
        #        return -1
        #    visited.add(q_from)
        #    if len(hm[q_from]) == 0:
        #        return -1
        #    for temp_to in hm[q_from]:
        #        edge = hm[q_from][temp_to]
        #        if temp_to == q_to:
        #            # print(acc, q_from, q_to, edge)
        #            return acc * edge
        #        else:
        #            res = dfs(visited, temp_to,q_to,acc * edge)
        #            if res != -1:
        #                return res

        #    return -1

        # for (q_from,q_to) in queries:
        #     visited=set()
        #     res.append(dfs(visited,q_from,q_to,1))

        return res