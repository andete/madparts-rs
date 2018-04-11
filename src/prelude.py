def flatten(S):
    if S == []:
        return S
    if isinstance(S[0], list):
        return flatten(S[0]) + flatten(S[1:])
    return S[:1] + flatten(S[1:])

class Element:
    def __init__(self):
        self.t = self.__class__.__name__
        
    def generate(self):
        import json
        return [json.dumps(self.__dict__)]
    
class Rect(Element): 
    def __init__(self, dx, dy):
        Element.__init__(self)
        self.x = 0.0
        self.y = 0.0
        self.dx = dx
        self.dy = dy

class Line(Element):
    def __init__(self, p1, p2, w=0.1):
        Element.__init__(self)
        (self.x1, self.y1) = p1
        (self.x2, self.y2) = p2
        self.w = w

class Text(Element):
    def __init__(self, txt, dy=1.0):
        Element.__init__(self)
        self.txt = txt
        self.dy = dy
        self.x = 0
        self.y = 0

# Arc

# Circle

# Pad

# Smd

# Polygon

# Text

# Hole ?
