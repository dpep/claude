[`polyid_uuid_like?`](https://github.com/dpep/polyid/blob/5fb38a8/lib/polyid/model.rb#L195) stripped every value before matching, and `strip` raises on invalid bytes. So `where(name: "\xFF abc")` raised [`ArgumentError`](https://docs.ruby-lang.org/en/3.4/ArgumentError.html) on every model. Rails usually rejects such params earlier, but imports and other inputs can carry them. It, and [`PolyId.uuid?`](https://github.com/dpep/polyid/blob/5fb38a8/lib/polyid.rb#L100), now check the encoding first.

`find("\xFF")` still raises, but that comes from ActiveRecord's own integer cast, not polyid.
