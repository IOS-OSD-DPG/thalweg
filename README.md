Thalweg Generator
=================

Generate a thalweg for an inlet

Procedure
---------

The core algorithm takes as input a list of lat/long/depth elements, as well as a list of "corners", which act as start and end points for each section along the inlet.
The first and last "corners" are the source and sink of the overall path, while each "corner" in between acts as the sink for one section and the source for the next.

