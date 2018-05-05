def footprint():
    name = Name("Texas_TSSOP-14_5x4.4x1.2mm_EP")
    name.y = 3.5
    
    reference = Reference()
    reference.y = -3.5
    
    fab = FFab(4.5, 5)
    fab.corner = 0.4
    
    crtyd = FCrtYd(6.6+0.7, 5+0.5)

    smd = Smd(1, (1.55, 0.25))
    smds = dual(smd, 5.6, 0.65, 14)
    #q = 5/0

    via = Pad(15, 0.63, 0.33)
    vias = dual(via, 0.95, 1.5, 6, 15)
    
    ep = Smd(15, (3.4, 5))
    ep.layers.remove("F.Mask")
    ep.layers.remove("F.Paste")
    paste = FPaste(2.46, 2.31)
    mask = FMask(2.46, 2.31)

    smds.append(ep)

    l1 = Line((-2.25, 2.6), (2.25, 2.6))
    l2 = Line((-3.5, -2.6), (2.25, -2.6))
    lines = [l1, l2]
    
    return [name, reference, fab, crtyd, mask, paste] + smds + vias + lines
 
