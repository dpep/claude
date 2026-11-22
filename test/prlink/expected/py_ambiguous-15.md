Fixes three regressions from #11, none of them released yet. First of three fix PRs from the bug hunt.

### Bugs
```python
class Base(abc.ABC):
    @abc.abstractmethod
    def kind(self): ...

    @ambiguous
    def name(self): ...
# TypeError: Base.name() missing 1 required positional argument: 'self'
```
The same failure hits `@dataclass`, `help()` / pydoc, and `@dataclass` on an `@ambiguous` class. Two related problems:
- `inspect.signature(any_ambiguous_fn)` raises `ValueError: no signature found for builtin`.
- `now = ambiguous(time.time)` or `stolen = Foo().name` in a class body gets `self` bound in. Plain Python doesn't do that.

### Cause
#11 put `__get__` on the delegating proxy and left the proxy itself in the class namespace. Anything that reads the namespace gets the proxy:
- `ABCMeta` scans every value for `__isabstractmethod__`;
- `dataclasses` checks for fields;
- pydoc lists members.

Each of those lookups is forwarded to the result, which means calling the method, with no `self`. And because the type now had `__get__`, `inspect` classified every ambiguous function as a method descriptor and took its builtin path.

### Fix
[`__set_name__`](https://github.com/dpep/py_ambiguous/blob/6f79955/ambiguous/ambiguous.py#L16) replaces `__get__`. Python calls it for every value in a class body while building the class, before `ABCMeta` or `@dataclass` look at the namespace. It swaps the proxy for a plain [`_AmbiguousMethod`](https://github.com/dpep/py_ambiguous/blob/6f79955/ambiguous/ambiguous.py#L73) descriptor, which doesn't forward anything. That's the descriptor #11 removed; [`ambiguous_class`](https://github.com/dpep/py_ambiguous/blob/7978fb8/ambiguous/ambiguous.py#L79) uses it again too. The swap only happens for plain, unbound functions, so builtins and already-bound methods stay unbound, as in plain Python.

**Limitations:**
- **Enum:** methods still can't use `@ambiguous` individually, because Enum inspects each value as it's assigned, before [`__set_name__`](https://github.com/dpep/py_ambiguous/blob/6f79955/ambiguous/ambiguous.py#L16) runs. Decorating the Enum class with `@ambiguous` works. The changelog says so.
- **Late assignment:** `Cls.x = ambiguous(f)` after the class exists doesn't bind `self`, since [`__set_name__`](https://github.com/dpep/py_ambiguous/blob/6f79955/ambiguous/ambiguous.py#L16) only runs during class creation.

Tests: [`test_class_introspection`](https://github.com/dpep/py_ambiguous/blob/6f79955/tests/test_ambiguous_object.py#L128), [`test_class_attribute_no_rebinding`](https://github.com/dpep/py_ambiguous/blob/6f79955/tests/test_ambiguous_object.py#L114) and [`test_signature`](https://github.com/dpep/py_ambiguous/blob/6f79955/tests/test_ambiguous_object.py#L164) fail before the fix. Passes on 3.12 and 3.14, and under `-O`.
