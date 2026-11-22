Fixes from a Rubyist review and a bug hunt. Each commit stands alone; most of them came with a regression spec that fails before the fix.

## Bugs

- **A rolled-back record could poison the cache.** The cache was written in `after_save`, inside the transaction. Rolled-back ids get reused, so `find(uuid)` could later return a different record. Writes now run after commit. Lookups made inside an open transaction are deferred to its commit as well.
  - The spec transaction is now non-joinable, as Rails' transactional fixtures are, so that `after_commit` fires. Rails runs commit callbacks for a transaction nested in a non-joinable one.
- **Uppercase uuids in string columns could never be found.** They were stored as typed, but lookups lowercase their input. They are now stored lowercase.
- **Malformed uuids could match an unrelated record.** A non-RFC uuid such as `"8f47...-0d7b-..."` fell through to the integer cast and matched id 8, which is an IDOR risk with `find(params[:id])`. Lookup input is now classified by shape.
- **`uuid_for("5")` returned nil on a cold cache.** String ids are now cast to the primary key type.
- **`unscoped.find(uuid)` didn't find rows hidden by the default scope** on a cache miss. The mapping is now resolved unscoped; the caller's query still applies its scope.
- **Garbage cache values were returned as-is.** This included 0.2.0's packed binary uuids. They are now treated as misses and overwritten.
- **[`PolyId.reset`](https://github.com/dpep/polyid/blob/f5873f9/lib/polyid.rb#L55) cleared the configured store**, which wipes the whole cache if that store is `Rails.cache`. It now just forgets the store.
- **Backfill:**
  - It could overwrite a uuid assigned while it ran; the update is now guarded on the column still being NULL.
  - It rejects invalid generated uuids.
- **A binary `uuid` primary key was cast as a polyid uuid**, nil-ing bytes that aren't RFC uuids. It is now left alone.

## Behaviour

- Translates uuids in more places: `"users.id"`, `account_id:`, `account:`, [`Set`](https://docs.ruby-lang.org/en/3.4/Set.html) values, and values with stray whitespace.
- `find` names an unknown uuid in its `RecordNotFound` message.
- Saving only writes the cache when a record gains its uuid, not on every update.
- [`is_uuid?`](https://github.com/dpep/polyid/blob/3d00802/lib/polyid.rb#L107) is renamed [`uuid?`](https://github.com/dpep/polyid/blob/3d00802/lib/polyid.rb#L99), with an alias.
- `rake polyid:backfill` drops its `uuid_attribute` argument. **This is a breaking change.**

## Specs

- "No query" assertions now count SQL. Stubbing `where`/[`find_by`](https://github.com/dpep/polyid/blob/f5873f9/lib/polyid/model.rb#L57) stopped proving anything once lookups went through `unscoped`.
- The bulk cache round-trip specs assert exact call counts.
- Removed:
  - CRUD examples that only tested ActiveRecord
  - a duplicate example
  - redundant setup
