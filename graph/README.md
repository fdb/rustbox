## NodeBox Graph Processor
Currently all NodeBox implementations will take a NodeBox graph and execute it directly. However, some benefit can be had from parsing the graph and doing passes over the output for optimization. For example, we can work better around conditional nodes (e.g. switch), or run time-independent nodes separately from time-dependendent nodes.

The general approach is to convert the graph to an intermediate format, e.g. bytecode, then perform several optimization passes over this format.