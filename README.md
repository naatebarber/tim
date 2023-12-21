### Create 3-dimensional geometry from an input string

Steps:
 - Convert string to hex
 - On an XYZ plane, create 16 circles (one for each hex char), each rotated around the z plane by (2Pi/16) * n radians, where n cooresponds to the index of the hex char.
 - Split each circle into M radial segments, where M is the length of the input
 - Each character from the input string is placed onto its cooresponding circle, with a radial segment offset of L (it's index in the input sequence);