Two descriptor bugs in [`ambiguous.py`](https://github.com/dpep/py_ambiguous/blob/486b957/ambiguous/ambiguous.py), plus the simplification that falls out of fixing them. Split from #2.

### Bug 1: an instance that evaluates as false gets the unbound function
```python
class Empty:
  def __len__(self): return 0

  @ambiguous.method
  def name(self): return 'empty'

Empty().name()  # TypeError: Empty.name() missing 1 required positional argument: 'self'
```
**Cause:** the descriptor tested `if obj:` to tell instance access from class access. [`Empty()`](https://github.com/dpep/py_ambiguous/blob/486b957/tests/test_ambiguous_object.py#L96) is falsy, so it was treated as `Empty.name`.

**Fix:** [`_AmbiguousMethod.__get__`](https://github.com/dpep/py_ambiguous/blob/486b957/ambiguous/ambiguous.py#L47) checks `obj is None`, the standard descriptor idiom.

### Bug 2: subclasses of an `@ambiguous` class get the parent as `cls`
```python
@ambiguous
class A:
  @classmethod
  def who(cls): return cls.__name__

class B(A): pass

B.who()  # 'A', expected 'B'
```
**Cause:** [`ambiguous_class`](https://github.com/dpep/py_ambiguous/blob/486b957/ambiguous/ambiguous.py#L73) used `inspect.getmembers(cls)`, which returns classmethods already bound to [`A`](https://github.com/dpep/py_ambiguous/blob/486b957/tests/test_ambiguous_object.py#L110). It then wrapped that bound method once, at decoration time, so every later lookup reused [`A`](https://github.com/dpep/py_ambiguous/blob/486b957/tests/test_ambiguous_object.py#L110). The `@ambiguous.classmethod` helper didn't have this bug because it binds at lookup.

**Fix:** keep the raw `classmethod` / `staticmethod` object and wrap it in [`_AmbiguousDescriptor`](https://github.com/dpep/py_ambiguous/blob/486b957/ambiguous/ambiguous.py#L55), which binds on each lookup against whichever class did the lookup.

### Simplification
- The three per-call descriptor class factories become those two module-level descriptor classes. [`ambiguous_classmethod`](https://github.com/dpep/py_ambiguous/blob/486b957/ambiguous/ambiguous.py#L69) is now just `_AmbiguousDescriptor(classmethod(func))`.
- [`ambiguous_class`](https://github.com/dpep/py_ambiguous/blob/486b957/ambiguous/ambiguous.py#L73) walks `vars(cls)`, the class's own attributes, instead of `getmembers` plus a lookup in `__dict__` and a type ladder. Inherited methods are still left alone, as before.
- Drops the unused [`AmbiguousType`](https://github.com/dpep/py_ambiguous/blob/a648990/ambiguous/ambiguous.py#L16) base and the `inspect` / `MethodType` imports.

Tests: [`test_falsy_instance`](https://github.com/dpep/py_ambiguous/blob/486b957/tests/test_ambiguous_object.py#L95) and [`test_subclass_classmethod`](https://github.com/dpep/py_ambiguous/blob/486b957/tests/test_ambiguous_object.py#L108) failed before this change.
