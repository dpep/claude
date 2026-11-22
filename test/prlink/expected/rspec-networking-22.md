The bundled manufacturer OUI files were corrupt:

- Every line of [`texas_instruments.txt`](https://github.com/dpep/rspec-networking/blob/264608b/lib/rspec/networking/mac_address/texas_instruments.txt) had a `mac_addresses.tsv:` grep prefix, so `from("texas instruments")` never matched.
- Several files had their leading zeros stripped by a spreadsheet. For example, Microsoft's `002248` was stored as `2248`. The hardcoded Xerox `000000` entry was papering over one case of this.
- [`apple.txt`](https://github.com/dpep/rspec-networking/blob/264608b/lib/rspec/networking/mac_address/apple.txt) and [`zyxel.txt`](https://github.com/dpep/rspec-networking/blob/264608b/lib/rspec/networking/mac_address/zyxel.txt) each contained a currency string (`AED 645.00`, `BEF 107.00`).
- The lists were stale; Apple, for example, had 1270 entries against 1593 in the registry now.

The new [`script/update_manufacturers`](https://github.com/dpep/rspec-networking/blob/264608b/script/update_manufacturers) regenerates the files from the IEEE OUI registry, using only MA-L assignments. Its output includes every well-formed old entry except 3 old "Texas Instruments" rows that actually belong to Texas Memory Systems, Texas Microsystems and Texas Digital Systems.

[`mac_address_manufacturers.rb`](https://github.com/dpep/rspec-networking/blob/264608b/lib/rspec/networking/mac_address_manufacturers.rb) now just loads the files. A new spec checks that every entry is 6 uppercase hex digits; it fails on the old data.
