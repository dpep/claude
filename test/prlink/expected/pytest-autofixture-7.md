Found in a bug hunt: each test and fixture ran against a **snapshot** of its module's globals. Correct plain-pytest tests failed with the plugin installed:
- `global COUNT; COUNT += 1` was lost, so the next test saw `0`
- a helper doing `global _state; _state = 1`, or a fixture setting a module global, wasn't visible to the test
- `monkeypatch.setattr(sys.modules[__name__], "TIMEOUT", 0)` had no effect

### Change
- [`_LiveGlobals`](https://github.com/dpep/pytest-autofixture/blob/fda8b3e/pytest_autofixture/__init__.py#L95) replaces the copy.
  - **Reads:** globals that aren't an exact dict take the slow path through [`__getitem__`](https://github.com/dpep/pytest-autofixture/blob/fda8b3e/pytest_autofixture/__init__.py#L114), which reads the module live and hides fixture objects (raising `KeyError` falls through to builtins, then to the fixture).
  - **Writes:** `STORE_GLOBAL` bypasses `__setitem__`, so `global` writes land in the view and [`write_back()`](https://github.com/dpep/pytest-autofixture/blob/fda8b3e/pytest_autofixture/__init__.py#L120) copies them to the module after the call.
  - **Dunders:** it copies only the module dunders, since C code reads those directly.
- Class bodies (and `exec`) use `LOAD_NAME`, which reads globals without [`__getitem__`](https://github.com/dpep/pytest-autofixture/blob/fda8b3e/pytest_autofixture/__init__.py#L114). So the builtins fallback ([`_module_value`](https://github.com/dpep/pytest-autofixture/blob/fda8b3e/pytest_autofixture/__init__.py#L46)) checks the live module before fixtures.
- [`_swap`](https://github.com/dpep/pytest-autofixture/blob/60dfb54/pytest_autofixture/__init__.py#L82) is simplified now that it builds the namespace itself.

Regression test covers all four cases plus a class body inside a test. Passes on Python 3.11, 3.12 and 3.14 locally; CI covers 3.13.

### Cost
The bug hunt's harness: 500 tests in a module with 5k globals, best of 3 runs, wall clock including startup.

| | time |
|---|---|
| without plugin | 0.27 s |
| snapshot | 0.43 s |
| live | 0.29 s |

Each global load is slower, about 0.3 µs against 0.04 µs, but the per-call copy of the whole module is gone.

Remaining gap: a helper that reads a global the test itself just wrote with `global` sees the old value until the test returns. That's rare, so no fix for now.

No changelog entry: the snapshot was introduced after 0.2.1 and never released.
