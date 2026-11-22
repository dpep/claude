Fixtures can now reference other fixtures by bare name, as tests already can. This enables RSpec's nested-`let` override pattern: an outer `user` fixture reads `role`, and an inner test class overrides only `role`.

### Change
[`pytest_fixture_setup`](https://github.com/dpep/pytest-autofixture/blob/cfbd1e9/pytest_autofixture/__init__.py#L81) wraps fixture setup the way [`pytest_runtest_call`](https://github.com/dpep/pytest-autofixture/blob/cfbd1e9/pytest_autofixture/__init__.py#L73) wraps the test. Both share [`_swap`](https://github.com/dpep/pytest-autofixture/blob/cfbd1e9/pytest_autofixture/__init__.py#L53), which temporarily replaces `fixturedef.func` with a rebound copy. Bound fixture methods stay bound, so pytest's `resolve_fixture_function` still binds them to the test instance.

The rebound copy resolves names against the fixture's own `SubRequest`, so:
- a session fixture that reads a function-scoped one raises `ScopeMismatch`, exactly as a declared parameter would
- overrides resolve from the requesting test's position

The hook is `tryfirst`, so it rebinds the user's function before pytest-asyncio wraps it in its sync runner. Without that, an async fixture body sees the module-level `FixtureFunctionDefinition` instead of the value, which fails silently. A test guards the ordering. Its inner conftest wraps `fixturedef.func` the way pytest-asyncio does and loads after us. The test fails without `tryfirst`, and it needs no pytest-asyncio dependency.

New README limitation, which was also true before this PR: an async test can't lazily request an async fixture, because pytest-asyncio can't start a loop inside the running one. Declare it as a parameter instead.

### Also in this PR
- **Tests split by need.** [`tests/test_autofixture.py`](https://github.com/dpep/pytest-autofixture/blob/cfbd1e9/tests/test_autofixture.py) holds plain tests that use the plugin directly, so they double as usage examples. [`tests/test_isolated.py`](https://github.com/dpep/pytest-autofixture/blob/cfbd1e9/tests/test_isolated.py) keeps pytester only for what needs its own session: state across tests, expected failures, and suite-wide hooks.
- **pytest's lookup errors come through.** Moving the tests surfaced a gap: on pytest 9.1+, a yield fixture that first uses a bare name after its `yield` got a misleading `NameError: ... no fixture by that name`. pytest refuses fixture lookups during teardown. [`__missing__`](https://github.com/dpep/pytest-autofixture/blob/5cac359/pytest_autofixture/__init__.py#L24) now turns a `FixtureLookupError` into `NameError` only when it has no message, so pytest's own explanation comes through. Added to the README limitations.
