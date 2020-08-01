#!/bin/env python3

import cpt_rust, inspect

cpt = cpt_rust.CPT()
cpt.train([1,2,3])
cpt.train([1,2,5])
cpt.train([1,5,8])
print(cpt.match_sequence([1,2,4]))
print(cpt.to_dot())

# print(inspect.getmembers(cpt_rust))