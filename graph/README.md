## NodeBox Graph Processor
Currently all NodeBox implementations will take a NodeBox graph and execute it directly. However, some benefit can be had from parsing the graph and doing passes over the output for optimization. For example, we can work better around conditional nodes (e.g. switch), or run time-independent nodes separately from time-dependendent nodes.

The general approach is to convert the graph to an intermediate format, e.g. bytecode, then perform several optimization passes over this format.

## Bytecode format
- Spreads (typed lists of values) are stored in a constant pool.
- Inspired by [Java bytecode](https://en.wikipedia.org/wiki/Java_bytecode_instruction_listings).

- `OP_SPREAD_LOAD`: Load a spread with the given index from the constant pool.
- `OP_CALL_NODE <NodeName>`: Call the node function with the given kind. Function will take all required arguments from the stack, execute, and place the result on the stack.

For speed, we also support singular values (ie. values not stored in spreads).