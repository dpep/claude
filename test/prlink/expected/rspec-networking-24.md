Fixes in [`be_an_ip_address`](https://github.com/dpep/rspec-networking/blob/1392908/lib/rspec/networking/ip_address.rb#L3):

- **CIDR ranges matched:** [`IPAddr.new`](https://docs.ruby-lang.org/en/3.4/IPAddr.html#method-c-new) parses `"10.0.0.0/8"` as a network, so it passed. Addresses with a non-host prefix are now rejected, via [`IPAddr#prefix`](https://docs.ruby-lang.org/en/3.4/IPAddr.html#method-i-prefix).
- **Error handling:** it rescues [`IPAddr::Error`](https://docs.ruby-lang.org/en/3.4/IPAddr/Error.html), the parent of both `InvalidAddressError` and `AddressFamilyError`, instead of only the first.
- **Messages:** RSpec now builds the failure messages from `description`. This fixes `nil` showing as `''` and the `"an IPv4 localhost "` description, which was missing "address".
- The README now documents `.v6`.

**Behavior change:** CIDR strings no longer match.


