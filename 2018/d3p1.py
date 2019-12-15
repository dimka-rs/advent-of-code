IN="./d3p1.in"

max_x = 0
max_y = 0

def add_claim(l):
    global max_x, max_y
    (id, x, y, w, h ) = map(int, l.split(' '))

    #print(id, x, y, w, h )
    if x + w > max_x:
        max_x = x + w

    if y + h > max_y:
        max_y = y + h



with open(IN, 'r') as f:
    for l in f:
        ## line format #1 @ 35,93: 11x13
        l = l.replace('#', '')
        l = l.replace(' @ ', ' ')
        l = l.replace(',', ' ')
        l = l.replace(':', '')
        l = l.replace('x', ' ')
        l = l.strip()
        add_claim(l)

    print("max x: ", max_x)
    print("max y: ", max_y)
