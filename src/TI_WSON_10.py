def footprint():
    name = Name("Texas_WSON-10_3x2x0.8mm_EP")
    name.y = 2.54
    
    reference = Reference()
    reference.y = -2.54
    
    fab = FFab(2, 3)
    
    crtyd = CrtYd(1.9+0.5+0.5, 3.5)

    smd = Smd(1, (0.5, 0.25))
    smds = dual(smd, 1.9, 0.5, 10)
    #q = 5/0

    # TODO: add thermal vias
    via1 = Pad(11, 0.4, 0.2)
    via2 = Pad(11, 0.4, 0.2)
    via2.y = 0.95
    via3 = Pad(11, 0.4, 0.2)
    via3.y = -0.95
    vias = [via1, via2, via3]
    
    ep = Smd(11, (0.84, 2.4))
    
    return [name, reference, fab, crtyd, ep] + smds + vias
 
