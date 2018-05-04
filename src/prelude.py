import copy, sys, traceback

def flatten(l):
    if l == []:
        return l
    if isinstance(l[0], list):
        return flatten(l[0]) + flatten(l[1:])
    return l[:1] + flatten(l[1:])

def handle_load_python(filename):
    try:
        exec(open(filename).read(), globals(), globals())
        return flatten(footprint())
    except:
        exc_type, exc_value, exc_traceback = sys.exc_info()
        message = "".join(traceback.format_exception(exc_type, exc_value, exc_traceback))
        e = PythonError(message)
        return [e]
        
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
        self.layer = "F.SilkS"

class FCrtYd(Rect):
    def __init__(self, dx, dy, w=0.05):
        Rect.__init__(self, dx, dy, w)
        self.layer = "F.CrtYd"

class FFab(Rect):
    def __init__(self, dx, dy, w=0.1):
        Rect.__init__(self, dx, dy, w)
        self.layer = "F.Fab"

class FPaste(Rect):
    def __init__(self, dx, dy, w=0.0):
        Rect.__init__(self, dx, dy, w)
        self.layer = "F.Paste"
        self.filled = True

class FMask(Rect):
    def __init__(self, dx, dy, w=0.0):
        Rect.__init__(self, dx, dy, w)
        self.layer = "F.Mask"
        self.filled = True
        
class Line(Element):
    def __init__(self, p1, p2, w=0.1):
        Element.__init__(self)
        (self.x1, self.y1) = p1
        (self.x2, self.y2) = p2
        self.w = w
        self.layer = "F.SilkS"

class Text(Element):
    def __init__(self, txt, dy=1.0, th=0.1):
        Element.__init__(self)
        self.txt = txt
        self.dy = dy
        self.x = 0
        self.y = 0
        self.thickness = th
        self.layer = "F.SilkS"

class Reference(Text):
    def __init__(self, txt="REF**", dy=1.0, th=0.15):
        Text.__init__(self, txt, dy, th)
        
class Name(Text):
    def __init__(self, txt, dy=1.0, th=0.15):
        Text.__init__(self, txt, dy, th)
        self.layer = "F.Fab" 

class Smd(Element):
    def __init__(self, name, s, p=(0,0)):
        Element.__init__(self)
        self.name = str(name)
        (self.dx, self.dy) = s
        (self.x, self.y) = p
        self.layers = ["F.Cu", "F.Paste", "F.Mask"]

    def at(self, name, x, y):
        n = copy.copy(self)
        n.name = str(name)
        n.x = x
        n.y = y
        return n

class Pad(Element):
    def __init__(self, name, s, d, p=(0,0)):
        Element.__init__(self)
        self.name = str(name)
        self.dx = s
        self.dy = s
        (self.x, self.y) = p
        self.drill = d
        self.layers = ["*.Cu", "*.Mask"]

    def at(self, name, x, y):
        n = copy.copy(self)
        n.name = str(name)
        n.x = x
        n.y = y
        return n

class Model(Element):
    def __init__(self, filename):
        Element.__init__(self)
        self.filename = filename

class PythonError(Element):
    def __init__(self, message):
        Element.__init__(self)
        self.message = message
        
# Arc

# Circle

# Pad

# Polygon

# Text

# Hole ?

def dual(pad, dx, dy, n, name=None):
    l = []
    n2 = int(n/2)
    dyn = float(dy)*n/2
    for i in range(0, n2):
        if name:
            name2 = name
        else:
            name2 = i+1
        l.append(pad.at(name2, -dx/2, -dyn/2 + dy/2 + dy*i))
    for i in range(0, n2):
        if name:
            name2 = name
        else:
            name2 = i+1+n2
        l.append(pad.at(name2, dx/2, dyn/2 - dy/2 - dy*i))
    return l

def single(pad, dy, n, name=None):
    l = []
    dyn = float(dy)*n
    for i in range(0, n):
        if name:
            name2 = name
        else:
            name2 = i+1
        l.append(pad.at(name2, 0, -dyn/2 + dy/2 + dy*i))
    return l
