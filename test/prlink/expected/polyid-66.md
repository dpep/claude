Fixes from a second bug hunt, aimed at code changed since 0.2.0. Every fix has a spec that fails before it.

- **Critical: an unresolved uuid matched an unrelated row.** An unknown scalar uuid translated to nil, which read as "no translation". The raw string then reached ActiveRecord and was integer-cast: `"0190abcd-..."` became 190. So `where(id:)`, [`find_by`](https://github.com/dpep/polyid/blob/2bc1e97/lib/polyid/model.rb#L56), `exists?` and `delete` could hit row 190. An unknown uuid inside an array became NULL and matched rows with no foreign key. Both now match nothing.
- **A `belongs_to ..., primary_key: :uuid` foreign key was translated to an integer id.** That column already holds uuids, so it's now left alone. Association classes are also only resolved when a value looks like a uuid. Before, `where(thing_id: 3)` raised if the association's `class_name` was missing.
- **Permitted params skipped translation.** `where` and [`find_by`](https://github.com/dpep/polyid/blob/2bc1e97/lib/polyid/model.rb#L56) now sanitize params first, as ActiveRecord does.
- **[`find_or_create_by`](https://github.com/dpep/polyid/blob/99b65d9/lib/polyid/relation.rb#L22) linked records to the wrong row.** It found with the translated foreign key but created with the raw uuid. The [`find_or_create_by`](https://github.com/dpep/polyid/blob/99b65d9/lib/polyid/relation.rb#L22) family now translates its attributes.
- **A destroy could leave a stale cache entry.** In `transaction { find(uuid).destroy! }`, the lookup's deferred cache write ran after the destroy's eviction, because a record's `after_commit` callbacks run before the transaction's own callbacks. The eviction is now queued on the transaction too, so the two run in order.
- **`find(uuid)` missed on a loaded `has_many ..., inverse_of:` association.** Rails searches the loaded records by id there and never reaches [`Relation#find`](https://github.com/dpep/polyid/blob/2bc1e97/lib/polyid/relation.rb#L4). It's now translated.
- **Small ones:**
  - `find(uuid, [uuid])` now flattens its arguments before translating.
  - [`uuids_for`](https://github.com/dpep/polyid/blob/2bc1e97/lib/polyid/model.rb#L87) no longer caches rows with no uuid.

Not fixed: when a uuid resolves to a row outside the relation's scope, the `RecordNotFound` message names the integer id rather than the uuid.
