Thalweg Generator
=================

Generate a thalweg for an inlet

Usage
-----

The primary usage is fairly simple:

	thalweg --data <path/to/NONNA-10/data> --corners <path/to/corners.txt>

This is roughly equivalent to

	thalweg --resolution 20 --prefix . --format dms --data <path/to/NONNA-10/data> --corners <path/to/corners.txt>

This will extract the source and sink from the file indicated by `--corners` and use them to extract a thalweg from the data provided.
`thalweg` will output the path to `<PREFIX>/path.txt`, and information that can be used to produce a section plot to `<PREFIX>/section.csv`.

The resolution argument controls how far the path segments will be from each other.
A larger resolution will likely result in fewer points along the thalweg.

The `--format` option controls the output of the thalweg file.
`dms` is roughly the same format as the ASCII files from NONNA-10.
`geojson` will produce a 3D `LineString` object, where the depth is represented as negative elevation.

The provided `section.py` script can consume `section.csv` to produce a section plot.

	python section.py /path/to/section.csv

Procedure
---------

The core algorithm takes as input a list of lat/long/depth elements, as well as a source (starting position) and a sink (ending position).
It computes the thalweg as the shortest path from the source to the sink through a graph representing the input bathymetry.
In order to incentivise taking deeper points in the graph, the weight of an edge from a node to its neighbor is defined to be `max_depth - neighbor.depth + 1`,
where `max_depth` is the maximum depth of any value present, and `neighbor.depth` is the depth of that particular neighbor.

The section information is produced using the distance between each point along the thalweg line.
Each point is converted into the total distance along the thalweg, keeping the depth as-is.
