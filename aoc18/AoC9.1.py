from collections import deque, defaultdict

def solution(max_players, last_marble):
    scores = defaultdict(int)
    circle = deque([0])

    for marble in range(1, last_marble + 1):
        if marble % 23 == 0:
            circle.rotate(7)
            scores[marble % max_players] += marble + circle.pop()
            circle.rotate(-1)
        else:
            circle.rotate(-1)
            circle.append(marble)

    return max(scores.values()) if scores else 0

if __name__ == '__main__':
    print(solution(9, 25))
    print(solution(10,1618))
    print(solution(13,7999))
    print(solution(17,1104))
    print(solution(21,6111))
    print(solution(411, 72059))
    print(solution(411, 7205900))