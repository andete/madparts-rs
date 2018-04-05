def flatten(S):
    if S == []:
        return S
    if isinstance(S[0], list):
        return flatten(S[0]) + flatten(S[1:])
    return S[:1] + flatten(S[1:])

class Element:
    def __init__(self, t):
        self.t = t
        
    def generate(self):
        return [self.__dict__]
    
class Rect(Element): 
    def __init__(self, dx, dy):
        Element.__init__(self, 'rect')
        self.x = 0.0
        self.y = 0.0
        self.dx = dx
        self.dy = dy

class Line(Element):
    def __init__(self, p1, p2, w=0.1):
        Element.__init__(self, 'line')
        (self.x1, self.y1) = p1
        (self.x2, self.y2) = p2
        self.w = w

# Arc

# Circle

# Pad

# Smd

# Polygon

# Text

# Hole ?
