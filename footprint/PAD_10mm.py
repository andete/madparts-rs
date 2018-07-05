def footprint():
    s = 10.0
    
    name = Name("PAD_10mm")
    name.y = s/2+1
    
    reference = Reference()
    reference.y = -s/2-1
    
    fab = FFab(s, s)
    fab.corner = 0.4
    
    crtyd = FCrtYd(s + 0.5, s + 0.5)

    smd = Smd(1, (s, s))
    smd.circle()
    smd.layers.remove("F.Mask")
    smd.layers.remove("F.Paste")
    
    return [name, reference, smd, fab, crtyd]
 
