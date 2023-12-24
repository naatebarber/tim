# TIM

Text to image toolkit and experimentation in rust.

Two goals for TIM:
 - Create efficient and reversible image representations of sequential text. (sequence -> image -> sequence)
   - The original idea was to create reversible image representations of text for use in a chaotic encryption scheme.
 - Create image geometry that can convey textual meaning to a CNN. This doesn't have to be reversible, but bonus points if it is.
   - This would be useful for experimentation in comparing the efficacy of a sequence to sequence model with a CNN that can derive lingual meaning from geometric representations of text

## Packages:

### `textual-geometry`: Create expressive geometry from an input string

**NHedron Encoder**

Attributes:  
 - Harder to decode (img to hex) because of irrational PI products being mapped to rational PX values.
 - Easier to model mathematically since data is encoded as elliptical geometry

Steps:
 - Convert string to hex
 - On an XYZ plane, create 16 circles (one for each hex char), each rotated around the z plane by (2Pi/16) * n radians, where n cooresponds to the index of the hex char.
 - Split each circle into M radial segments, where M is the length of the input
 - Each character from the input string is placed onto its cooresponding circle, with a radial segment offset of L (it's index in the input sequence);

**Spiral Encoder**

Attributes:
 - Predefined input seq size defined by W, H of the image
 - Relatively easy to decode after encode
 - Less mathematically easy to model, as the creation of the geometry is programmatic and not sinusoidal
 
DECODE:
 - TODO:
   - Remove bitmap padding and just account for PX allocation correctly
   - Experiment with search & sparse points instead of creating dense Points
   - There is overlap at the spiral edges. Fix that so decoding can work properly. (DONE)
   - Dim is off at 257 vs 256. Fix. (DONE)
   - One extra character is being added for wrapping sequences, fix.