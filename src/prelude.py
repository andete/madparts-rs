import copy, sys, traceback

def flatten(l):
    if l == []:
        return l
    if isinstance(l[0], list):
        return flatten(l[0]) + flatten(l[1:])
    return l[:1] + flatten(l[1:])

def handle(footprint_fun):
    try:
        l = footprint_fun()
        return flatten(l)
    except:
        exc_type, exc_value, exc_traceback = sys.exc_info()
        return "".join(traceback.format_exception(exc_type, exc_value, exc_traceback))

class Element:
    def __init__(self):
        self.t = self.__class__.__name__
        
    def generate(self):
        import json
        return [json.dumps(self.__dict__)]
    
class Rect(Element): 
    def __init__(self, dx, dy, w=0.1):
        Element.__init__(self)
        self.x = 0.0
        self.y = 0.0
        self.dx = dx
        self.dy = dy
        self.w = w
        self.filled = False
        self.layer = "FSilkS"

class CrtYd(Rect):
    def __init__(self, dx, dy, w=0.05):
        Rect.__init__(self, dx, dy, w)
        self.layer = "CrtYd"

class FFab(Rect):
    def __init__(self, dx, dy, w=0.1):
        Rect.__init__(self, dx, dy, w)
        self.layer = "FFab"
        
class Line(Element):
    def __init__(self, p1, p2, w=0.1):
        Element.__init__(self)
        (self.x1, self.y1) = p1
        (self.x2, self.y2) = p2
        self.w = w
        self.layer = "FSilkS"

class Text(Element):
    def __init__(self, txt, dy=1.0):
        Element.__init__(self)
        self.txt = txt
        self.dy = dy
        self.x = 0
        self.y = 0

class Reference(Text):
    def __init__(self, txt="REF**", dy=1.0):
        Text.__init__(self, txt, dy)
        
class Name(Text):
    def __init__(self, txt, dy=1.0):
        Text.__init__(self, txt, dy)

class Smd(Element):
    def __init__(self, name, s, p=(0,0)):
        Element.__init__(self)
        self.name = str(name)
        (self.dx, self.dy) = s
        (self.x, self.y) = p

    def at(self, name, x, y):
        n = copy.copy(self)
        n.name = str(name)
        n.x = x
        n.y = y
        return n
        
        
# Arc

# Circle

# Pad

# Polygon

# Text

# Hole ?

def dual(pad, dx, dy, n):
    l = []
    n2 = int(n/2)
    dyn = float(dy)*n/2
    for i in range(0, n2):
        l.append(pad.at(i+1, -dx/2, -dyn/2 + dy/2 + dy*i))
    for i in range(0, n2):
        l.append(pad.at(i+1+n2, dx/2, dyn/2 - dy/2 - dy*i))
    return l
