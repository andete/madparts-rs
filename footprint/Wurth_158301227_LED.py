def footprint():
    name = Name("LED_Wurth_158301227")
    name.y = 5
    
    reference = Reference()
    reference.y = -5
    
    fab = FFab(3, 1.4)
    fab.corner = 0.4
    
    w = 1.2+0.55+2.0
    crtyd = FCrtYd(w+0.5, 1.4+0.5)

    smd1 = Smd(1, (2.0, 0.8))
    smd1.x = -2/2 + w/2
    smd1.x = - smd1.x
    smd2 = Smd(2, (1.2, 0.8))
    smd2.x = 1.2/2 - w/2
    smd2.x = -smd2.x
    smds = [smd1, smd2]

    l1 = Line((-3/2, 1.4/2), (3/2, 1.4/2))
    l2 = Line((-3/2-0.3, -1.4/2), (3/2, -1.4/2))
    lines = [l1, l2]
    
    return [name, reference, fab, crtyd] + smds + lines
 
