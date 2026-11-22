The callbacks read the uuid unguarded, so a record loaded with a partial select raised `ActiveModel::MissingAttributeError` on every polyid model: `User.select(:id, :name).find(1).update!(name: "x")`, `.valid?`, and `.destroy`. A failed destroy also rolled back, so the row survived.

The validation and canonicalise callbacks now skip when the uuid isn't loaded. Eviction drops just the id key in that case; the uuid key it can't name is harmless under the documented cache contract, since it resolves to a missing row.
