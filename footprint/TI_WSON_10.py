def footprint():
    name = Name("Texas_WSON-10_3x2x0.8mm_EP")
    name.y = 2.54
    
    reference = Reference()
    reference.y = -2.54
    
    fab = FFab(2, 3)
    fab.corner = 0.4
    
    crtyd = FCrtYd(1.9+0.5+0.5, 3.5)

    smd = Smd(1, (0.5, 0.25))
    smds = dual(smd, 1.9, 0.5, 10)
    #q = 5/0

    via = Pad(11, 0.5, 0.2)
    vias = single(via, 0.95, 3, 11)
    
    ep = Smd(11, (0.84, 2.4))

    l1 = Line((-1, 1.65), (1, 1.65))
    l2 = Line((-1.35, -1.65), (1, -1.65))
    lines = [l1, l2]
    
    return [name, reference, fab, crtyd, ep] + smds + vias + lines
 
