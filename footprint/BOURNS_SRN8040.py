def footprint():
    name = Name("L_Bourns_SRN8040")
    name.y = 5
    
    reference = Reference()
    reference.y = -5
    
    fab = FFab(8, 8)
    fab.corner = 0.4
    
    crtyd = FCrtYd(8+0.5, 8+0.5)

    smd = Smd(1, (2.6, 8.2))
    smds = dual(smd, 5.6, 0.65, 2)

    l1 = Line((-4, 4.2), (4, 4.2))
    l2 = Line((-4, -4.2), (4, -4.2))
    lines = [l1, l2]
    
    return [name, reference, fab, crtyd] + smds + lines
 
