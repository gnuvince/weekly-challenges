from collections import deque
import sys
from pprint import pprint

def get(m, i, j):
    if i >= 0 and i < len(m):
        if j >= 0 and j < len(m[i]):
            return m[i][j]
    return None

def connect(matrix, rows, cols):
    g = {}
    for i in range(rows):
        for j in range(cols):
            if not matrix[i][j]:
                continue
            neighbors = []
            if get(matrix, i-1, j): neighbors.append((i-1, j))
            if get(matrix, i+1, j): neighbors.append((i+1, j))
            if get(matrix, i, j-1): neighbors.append((i, j-1))
            if get(matrix, i, j+1): neighbors.append((i, j+1))
            g[(i, j)] = neighbors
    return g

def bfs(g, start):
    seen = {start}
    queue = deque([start])
    while queue:
        curr = queue.popleft()
        for neighbor in g[curr]:
            if neighbor not in seen:
                seen.add(neighbor)
                queue.append(neighbor)
    return seen

def connected_components(g):
    components = 0
    seen = set()
    for node in g:
        if node in seen:
            continue
        cluster = bfs(g, node)
        components += 1
        seen |= cluster
    return components

rows, cols = map(int, sys.stdin.readline().split())
matrix = [[0 for _ in range(cols)] for _ in range(rows)]
for i in range(rows):
    line = sys.stdin.readline()
    for j in range(cols):
        if line[j] != "0":
            matrix[i][j] = 1
g = connect(matrix, rows, cols)
print(connected_components(g))
