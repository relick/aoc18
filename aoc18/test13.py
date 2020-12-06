def part_a():
    track = []
    with open('test13a.txt') as data:
        for line in data:

            track.append([c for c in line.strip('\n')])

    dirr = ['^', 'v', '>', '<']
    carts = []
    #(dir, row, col, turn)
    for i in range(len(track)):
        for j in range(len(track[i])):
            if track[i][j] in dirr:
                carts.append([track[i][j], i, j, 0])

                if track[i][j] == '^' or track[i][j] == 'v':
                    track[i][j] = '|'
                else:
                    track[i][j] = '-'

    while True:
        move = set()
        for cart in carts:
            y,x = cart[1:3]

            moving(cart, track)

            if (x,y) in move:
                return (x,y)
            else:
                move.add((x,y))

def part_b():
    track = []
    with open('test13b.txt') as data:
        for line in data:
            track.append([c for c in line.strip('\n')])

    dirr = ['^', 'v', '>', '<']

    carts = []
    for i in range(len(track)):
        for j in range(len(track[i])):
            if track[i][j] in dirr:
                carts.append([track[i][j], i, j, 0])

                if track[i][j] == '^' or track[i][j] == 'v':
                    track[i][j] = '|'
                else:
                    track[i][j] = '-'
    
    while True:
        for i in range(len(track)):
            for j in range(len(track[i])):
                for cart in carts:
                    if [i,j] == cart[1:3]:
                        cart = moving(cart,track)

                    if len(carts) != 1:
                        rem_carts = list(carts)
                        for i in range(len(carts)):
                            for j in range(i+1, len(carts)):
                                if (carts[i][1] == carts[j][1]) and (carts[i][2] == carts[j][2]):
                                    rem_carts.remove(carts[i])
                                    rem_carts.remove(carts[j])
                        carts = list(rem_carts)
                    else:
                        y,x = carts[0][1:3]
                        return(x,y)

def moving(cart, track):
    #turn_rotation = left, straight, right
    y,x = cart[1:3]
    n, s, e, w = ['^', 'v', '>', '<']
    if cart[0] == n:
        cart[1] -= 1
        if track[y-1][x] == '/':
            cart[0] = '>'
        elif track[y-1][x] == '\\':
            cart[0] = '<'
        elif track[y-1][x] == '+':
            cart[3] += 1
            if cart[3] % 3 == 1:
                cart[0] = '<'
            elif cart[3] % 3 == 2:
                cart[0] = '^'
            else:
                cart[0] = '>'

    elif cart[0] == s:
        cart[1] += 1
        if track[y+1][x] == '\\':
            cart[0] = '>'
        elif track[y+1][x] == '/':
            cart[0] = '<'
        elif track[y+1][x] == '+':
            cart[3] += 1
            if cart[3] % 3 == 1:
                cart[0] = '>'
            elif cart[3] % 3 == 2:
                cart[0] = 'v'
            else:
                cart[0] = '<'

    elif cart[0] == e:
        cart[2] += 1
        if track[y][x+1] == '\\':
            cart[0] = 'v'
        if track[y][x+1] == '/':
            cart[0] = '^'
        elif track[y][x+1] == '+':
            cart[3] += 1
            if cart[3] % 3 == 1:
                cart[0] = '^'
            elif cart[3] % 3 == 2:
                cart[0] = '>'
            else:
                cart[0] = 'v'

    elif cart[0] == w:
        cart[2] -= 1
        if track[y][x-1] == '\\':
            cart[0] = '^'
        if track[y][x-1] == '/':
            cart[0] = 'v'
        elif track[y][x-1] == '+':
            cart[3] += 1
            if cart[3] % 3 == 1:
                cart[0] = 'v'
            elif cart[3] % 3 == 2:
                cart[0] = '<'
            else:
                cart[0] = '^'
    
    return cart

if __name__ == '__main__':
    print(part_a())
    print(part_b())