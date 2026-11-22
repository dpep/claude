Fixes in [`be_a_mac_address`](https://github.com/dpep/rspec-networking/blob/ddf572d/lib/rspec/networking/mac_address.rb#L3):

- **Partial matches:** the regexes used `^`/`$`, so `"junk\n00:00:00:00:00:00"` matched. They're now anchored with `\A`/`\z`.
- **IPv4 strings matched:** `expect("192.168.100.200").to be_a_mac_address` passed, because of the `MMM.MMM.SSS.SSS` form. That isn't a real MAC notation. It's replaced with Cisco's `MMMM.MMSS.SSSS` ([`EUI48`](https://github.com/dpep/rspec-networking/blob/28c1f48/lib/rspec/networking/mac_address.rb#L7)), which used to be rejected. `.from`/`.for` take colon or dash halves ([`HALF`](https://github.com/dpep/rspec-networking/blob/28c1f48/lib/rspec/networking/mac_address.rb#L13)).
- **64-bit form:** it now accepts dashes, like the 48-bit form.
- **`from(123)`:** it raised [`NoMethodError`](https://docs.ruby-lang.org/en/3.4/NoMethodError.html); it now raises [`ArgumentError`](https://docs.ruby-lang.org/en/3.4/ArgumentError.html). `.from` now resolves the manufacturer to a prefix set once. Before, the exact-prefix vs. lookup decision was made a second time with a looser regex.
- **Messages:** RSpec now builds the failure messages from `description`. This fixes `nil` showing as `''` and test names reading "is expected to a MAC address". The [`a_mac_address`](https://github.com/dpep/rspec-networking/blob/28c1f48/lib/rspec/networking/mac_address.rb#L105) alias strips the "be " so composed messages still read `include {addr: (a MAC address)}`.

**Behavior change:** `MMM.MMM.SSS.SSS` strings no longer match, so a release needs at least a minor bump. Each fix has a regression spec that fails on `main`. The spec also fixes an example meant to test dashes, which never did (`gsub("-", ":")` on colon-separated Faker output).
