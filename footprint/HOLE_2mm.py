def footprint():
    name = Name("HOLE_2mm")
    name.y = 3
    
    reference = Reference()
    reference.y = -3
    
    fab = FFab(2, 2)
    fab.corner = 0.4
    
    crtyd = FCrtYd(2.5, 2.5)

    hole = Hole(1, 2, 2)
    
    return [name, reference, hole, fab, crtyd]
 
