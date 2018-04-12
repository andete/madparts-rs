def footprint():
    name = Name("C_0402")
    name.y = 1.27
    
    reference = Reference()
    reference.y = -1.27
    
    fab = Rect(1, 0.5)
    fab.layer = "FFab"
    
    crtyd = Rect(2, 0.8, 0.05)
    crtyd.layer = "CrtYd"

    s1 = Line((0.25, -0.47), (-0.25, -0.47), 0.12)
    s2 = Line((0.25, 0.47), (-0.25, 0.47), 0.12)

    p1 = Smd("1", (-0.55, 0), (0.6, 0.5))
    p2 = Smd("2", (0.55, 0), (0.6, 0.5))
    
    return [name, reference, fab, crtyd, s1, s2, p1, p2]
 