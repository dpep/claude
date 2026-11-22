[`thing_or_things`](https://github.com/dpep/py_ambiguous/blob/a5ae4ec/ambiguous/thing_or_things.py#L9) fixes from the last bug hunt:

- **`self` was picked as the thing.** With keyword-only things (#17), `def get(self, *, ids)` treated `self` as the thing because it was the only positional parameter, and the call then failed with "'int' object is not iterable". Candidates now skip `self` / `cls`. As a side effect, `def get(self, ids)` no longer needs `@thing_or_things('ids')`.
- **A frozenset was treated as one thing.** `ident(frozenset({1, 2}))` returned the frozenset itself with no error. It now counts as many things, like a set. Range, generators and dict views still count as single things, as the docstring says. The README PR will spell this out.
- **Unhashable things failed after `fn` had already run**, and the error didn't name the function. They're now checked before the call.
- **Missing and extra keys were listed in set order**, which differed between 3.12 and 3.14. They're now listed in the order of the input and the result.
- **Async functions** got "must return a dict or None, got coroutine" plus a never-awaited warning. They're now rejected at decoration time.

Tests: [`test_methods`](https://github.com/dpep/py_ambiguous/blob/fae9c87/tests/test_thing_or_things.py#L172), [`test_method_hint`](https://github.com/dpep/py_ambiguous/blob/a5ae4ec/tests/test_thing_or_things.py#L172), [`test_async`](https://github.com/dpep/py_ambiguous/blob/fae9c87/tests/test_thing_or_things.py#L198), frozensets, and [`test_unhashable_thing`](https://github.com/dpep/py_ambiguous/blob/fae9c87/tests/test_thing_or_things.py#L339) and [`test_key_order`](https://github.com/dpep/py_ambiguous/blob/fae9c87/tests/test_thing_or_things.py#L352).
