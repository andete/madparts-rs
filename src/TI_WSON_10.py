def footprint():
    name = Name("Texas_WSON-10_3x2x0.8mm_EP")
    name.y = 2.54
    
    reference = Reference()
    reference.y = -2.54
    
    fab = FFab(2, 3)
    
    crtyd = CrtYd(1.9+0.5+0.5, 3.5)

    pad = Smd(1, (0.5, 0.25))
    pads = dual(pad, 1.9, 0.5, 10)
    
    ep = Smd(11, (0.84, 2.4))
    
    return [name, reference, fab, crtyd, ep] + pads
 
