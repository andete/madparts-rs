import copy

"""Base class for footprint elements."""
class Element:
    """Base class for footprint elements.
    
    Provides code for conversion to json.
    Normally not used directly by end-users.
    """
    
    def __init__(self):
        self.t = self.__class__.__name__
        
    def generate(self):
        import json
        return [json.dumps(self.__dict__)]

class Rect(Element):
    """Rectangular shaped Element

    Keyword arguments:
        dx (float): x size
        dy (float): y size
        w (float): line width (default 0.1)

    Attributes:
        x (float): x position
        y (float): y position
        dx (float): x size
        dy (float): y size
        w  (float): line width
        filled (boolean): if the rectangular should be filled (default False)
        layer (string): Kicad layer to use (default "F.SilkS")
    """
    
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
    """
    Rectangular Element that uses the "F.CrtYd" Kicad layer
    """
    
    def __init__(self, dx, dy, w=0.05):
        Rect.__init__(self, dx, dy, w)
        self.layer = "F.CrtYd"

class FFab(Rect):
    """
    Rectangular Element that uses the "F.Fab" Kicad layer
    """
    
    def __init__(self, dx, dy, w=0.1):
    
        Rect.__init__(self, dx, dy, w)
        self.layer = "F.Fab"

class FPaste(Rect):
    """
    Rectangular Element that uses the "F.Paste" Kicad layer
    """
    
    def __init__(self, dx, dy, w=0.0):
        Rect.__init__(self, dx, dy, w)
        self.layer = "F.Paste"
        self.filled = True

class FMask(Rect):
    """
    Rectangular Element that uses the "F.Mark" Kicad layer
    """
    def __init__(self, dx, dy, w=0.0):
        Rect.__init__(self, dx, dy, w)
        self.layer = "F.Mask"
        self.filled = True
        
class Line(Element):
    """
    Line shaped Element

    Keyword arguments:
        p1 (float,float): point 1
        p1 (float,float): point 2
        w (float): line width (default 0.1)

    Attributes:
        x1 (float): x position of point 1
        y1 (float): y position of point 1
        x2 (float): x position of point 2
        y2 (float): y position of point 2
        w  (float): line width
        layer (string): Kicad layer to use (default "F.SilkS")

    """
    
    def __init__(self, p1, p2, w=0.1):
        Element.__init__(self)
        (self.x1, self.y1) = p1
        (self.x2, self.y2) = p2
        self.w = w
        self.layer = "F.SilkS"

class Text(Element):
    """
    Text Element

    Keyword arguments:
        txt (string): text to display
        dy (float): size of text (default 1.0)
        th (float): thickness of text (default 0.1)

    Attributes:
        text (string): text to display
        dy (float): size of text
        x (float): x position
        y (float): y position
        thickness (float): thickness of text
        layer (string): Kicad layer to use (default "F.SilkS")

    """
     
    def __init__(self, txt, dy=1.0, th=0.1):
        Element.__init__(self)
        self.txt = txt
        self.dy = dy
        self.x = 0
        self.y = 0
        self.thickness = th
        self.layer = "F.SilkS"

class Reference(Text):
    """
    Text Element to be used for the kicad Reference field.

    Keyword arguments:
        txt (string): text to display (default "REF**")
        dy (float): size of text (default 1.0)
        th (float): thickness of text (default 0.15)
    """
     
    def __init__(self, txt="REF**", dy=1.0, th=0.15):
        Text.__init__(self, txt, dy, th)
        
class Name(Text):
    """
    Text Element to be used for the kicad Reference field.

    Keyword arguments:
        txt (string): name to display
        dy (float): size of text (default 1.0)
        th (float): thickness of text (default 0.15)

    This will be placed on the F.Fab layer by default.
    """
         
    def __init__(self, txt, dy=1.0, th=0.15):
        Text.__init__(self, txt, dy, th)
        self.layer = "F.Fab" 

class Smd(Element):
    """SMD Pad Element

    Keyword arguments:
        name (string): name of the pad
        s (float,float): size of the pad
        p (float,float): location of the pad (default: (0,0))

    Attributes:
        name (string): name of the pad
        x (float): x position
        y (float): y position
        dx (float): x size
        dy (float): y size
        layers ([string]): Kicad layers to use (default  ["F.Cu", "F.Paste", "F.Mask"])
    """
    
    def __init__(self, name, s, p=(0,0)):
        Element.__init__(self)
        self.name = str(name)
        (self.dx, self.dy) = s
        (self.x, self.y) = p
        self.layers = ["F.Cu", "F.Paste", "F.Mask"]
        self.shape = "rect"

    def at(self, name, x, y):
        """Create a copy of this SMD Pad with a new name and location"""
        
        n = copy.copy(self)
        n.name = str(name)
        n.x = x
        n.y = y
        return n

    def circle(self):
        self.shape = "circle"

class Pad(Element):
    """PTH Pad Element

    Keyword arguments:
        name (string): name of the pad
        s (float): size of the pad
        p (float,float): location of the pad (default: (0,0))
        d (float): drill size

    Attributes:
        name (string): name of the pad
        x (float): x position
        y (float): y position
        dx (float): x size
        dy (float): y size
        layers ([string]): Kicad layers to use (default  ["*.Cu", "*.Mask"])
    """
      
    def __init__(self, name, s, d, p=(0,0)):
        Element.__init__(self)
        self.name = str(name)
        self.dx = s
        self.dy = s
        (self.x, self.y) = p
        self.drill = d
        self.layers = ["*.Cu", "*.Mask"]
        self.plated = True

    def at(self, name, x, y):
        """Create a copy of this PTH Pad with a new name and location"""
        
        n = copy.copy(self)
        n.name = str(name)
        n.x = x
        n.y = y
        return n

    def non_plated(self):
        self.plated = False

class Hole(Pad):
    def __init__(self, name, s, d, p=(0,0)):
        Pad.__init__(self, name, s, d, p)
        self.non_plated()
    
        
class Model(Element):
    """TODO
    """
    
    def __init__(self, filename):
        Element.__init__(self)
        self.filename = filename

# Arc

# Circle

# Pad

# Polygon

# Text

def dual(pad, dx, dy, n, name=None):
    """Create a double row of n pads from a pad template
    """
    
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
    """Create a single row of n pads from a pad template
    """
    
    l = []
    dyn = float(dy)*n
    for i in range(0, n):
        if name:
            name2 = name
        else:
            name2 = i+1
        l.append(pad.at(name2, 0, -dyn/2 + dy/2 + dy*i))
    return l

### internal functions and classes

class PythonError(Element):
    """Element used to signal a problem loading or running the 
    python script.
    """
    
    def __init__(self, message):
        Element.__init__(self)
        self.message = message

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
        import sys, traceback
        exc_type, exc_value, exc_traceback = sys.exc_info()
        message = "".join(traceback.format_exception(exc_type, exc_value, exc_traceback))
        e = PythonError(message)
        return [e]
