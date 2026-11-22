Fixes the bugs from a bug hunt and a Python-idioms review that don't change any intended behavior. There's one commit per fix, and each comes with tests that fail without it.

- **Mappings and sets.** `pluck` now dispatches on `collections.abc.Mapping` and `Set` instead of `dict` and `set`. Before, `pluck(MappingProxyType({'a': [1]}), 0)` silently returned `['a']` and `ChainMap` crashed. The wildcard only calls `.values()` on Mappings, so an iterable whose `values` attribute isn't a method no longer breaks.
- **Method-wrappers.** `inspect.isroutine` replaces the hand-written list of function types. `pluckit('abc', '.__len__')` used to return the method object; it now returns 3. Classes stored as attributes still aren't called.
- **Parse, then evaluate.** [`parse_path`](https://github.com/dpep/py_pluckit/blob/305cf61/pluckit/pluckit.py#L48) turns the whole path into steps, and [`_walk`](https://github.com/dpep/py_pluckit/blob/305cf61/pluckit/pluckit.py#L83) applies them. Before, the regex ran inside the evaluation loop. This fixes:
  - `assert obj == res` called the object's `__eq__`, so `pluckit(float('nan'), 'real')` and numpy-like objects crashed, except under `python -O`. The bare-name heuristic now applies only to the first step because of how the path is parsed, so no assert is needed.
  - validation depended on the data: `pluck([], '!!bad')` returned `[]` and `pluckit([], '*.a.')` passed. `pluck` validates up front now. An empty path is an error.
  - quoted keys can contain `]`, `.` or `[`: `'["a].b"]'` used to look up `"a`. `'["]'` used to look up the empty string.
  - whitespace inside brackets is ignored: `'[ 0 ]'` now works.

  The redundant `int` branch and `from types import *` are gone, and `__pluckPath` is now [`_walk`](https://github.com/dpep/py_pluckit/blob/305cf61/pluckit/pluckit.py#L83). The double underscore never made it private, because Python only does that inside a class.
- **[`native.uninstall`](https://github.com/dpep/py_pluckit/blob/78b0fa1/pluckit/native.py#L30)** skips types it didn't install. Before, it raised `KeyError: 'pluck'` from forbiddenfruit.
- **Exports.** `__all__` now lists the Pluckable classes, so `from pluckit import *` includes them.

Not in this PR, because each one changes current behavior and needs a decision:
- Should attributes that hold functions be called? Today `SimpleNamespace(cb=len)` with `'cb'` crashes calling `len()`.
- Should `'a.b'` fall back to a key lookup on dicts?
- A bare name that matches a method calls it, so `pluckit(lst, 'pop')` mutates the list.
- Should `pluck` on a `str` raise instead of plucking each character?
- Should a `None` handle raise instead of returning `None` for every item?
- Should [`native.py`](https://github.com/dpep/py_pluckit/blob/305cf61/pluckit/native.py) keep patching builtins on import, or go away entirely?
