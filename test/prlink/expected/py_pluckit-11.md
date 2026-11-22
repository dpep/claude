The regex used to run inside the evaluation loop, so a syntax error was only caught if the data got that far: `pluck([], '!!bad')` returned `[]`, and `pluckit([], '*.a.')` passed. Now [`parse_path`](https://github.com/dpep/py_pluckit/blob/7ddc944/pluckit/pluckit.py#L40) turns the whole path into steps (cached), [`_walk`](https://github.com/dpep/py_pluckit/blob/7ddc944/pluckit/pluckit.py#L70) applies them, and `pluck` validates up front.

The regex is unchanged; only when it runs moves. I compared the old and new code on 240k random path/data combinations. The only difference was the new code raising ValueError where the old code skipped validation.

Part 8 of 11 in a stack (based on #10). Merge them in order.