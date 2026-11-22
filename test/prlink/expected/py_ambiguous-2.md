Fixes five bugs the suite didn't catch, simplifies the code they lived in, and refreshes CI. Each fix commit carries a regression test that failed first, plus its [`CHANGES.txt`](https://github.com/dpep/py_ambiguous/blob/6b67bff/CHANGES.txt) line under Unreleased. Reviewing commit by commit is easiest.

### Bugs fixed
- **Falsy instances:** `@ambiguous.method` checked `if obj:`, so an instance with `__len__() == 0` got the unbound function. [`_AmbiguousMethod`](https://github.com/dpep/py_ambiguous/blob/6b67bff/ambiguous/ambiguous.py#L43) now checks `is None`.
- **Subclass classmethods:** on an `@ambiguous` class, classmethods were bound at decoration time, so `B.who()` on a subclass got [`A`](https://github.com/dpep/py_ambiguous/blob/6b67bff/tests/test_ambiguous_object.py#L111) as `cls`. [`_AmbiguousDescriptor`](https://github.com/dpep/py_ambiguous/blob/6b67bff/ambiguous/ambiguous.py#L55) binds at lookup, and [`ambiguous_class`](https://github.com/dpep/py_ambiguous/blob/6b67bff/ambiguous/ambiguous.py#L73) walks `vars(cls)` instead of `inspect.getmembers` plus a type ladder.
- **In-place ops:** `x = n; x += 1` raised `AttributeError` for int/str results. The in-place ops are now excluded, so Python falls back to `x = x + 1`; each access re-evaluates the function, so there is nothing to mutate anyway. Also stops scanning the `operator` module, which only added non-protocol names.
- **[`thing_or_things`](https://github.com/dpep/py_ambiguous/blob/be1ea27/ambiguous/thing_or_things.py#L10) keyword/default args:** `itself(args=[1, 2])` and default-valued things crashed. The call is now bound with `inspect.signature`, replacing the call-it-to-raise trick and its `assert` (stripped under `-O`).
- **`@decorator` returning its target:** `update_wrapper(fn, fn)` made `fn` its own `__wrapped__`, so `inspect.signature` raised "wrapper loop". Fixed, and the three arg accumulators fold into one `collect`.
- Also fixes a [`test_return_type`](https://github.com/dpep/py_ambiguous/blob/be1ea27/tests/test_thing_or_things.py#L216) assertion (`assertTrue(dict, ...)`) that could never fail.

### Other changes
- **Callable module:** [`_CallableModule`](https://github.com/dpep/py_ambiguous/blob/6b67bff/ambiguous/__init__.py#L30) is assigned to the module's `__class__`, the supported mechanism, replacing the stand-in wrapper object with its spoofed `__class__`. `ambiguous` is now a real module, so `reload` and `isinstance(ambiguous, ModuleType)` behave normally.
- **Breaking-ish:** [`ambiguous()`](https://github.com/dpep/py_ambiguous/blob/be1ea27/ambiguous/ambiguous.py#L115) on an unsupported type raises `TypeError` instead of `NotImplementedError`. Noted in the changelog.
- **CI:** 3.15 added to the matrix and tox; checkout/setup-python v7, upload-artifact v7, download-artifact v8; plain `pull_request` trigger; read-only default token; Dependabot + auto-merge from `python_package_template`.
- **Tests and housekeeping:** dropped the `sys.path` hacks, shebangs, a star import; extended [`.gitignore`](https://github.com/dpep/py_ambiguous/blob/6b67bff/.gitignore).
- **README:** documents `@ambiguous` on classes plus two limitations (results don't persist mutation; pass callable `@decorator` options by keyword).

Next release would be 0.5.2.
