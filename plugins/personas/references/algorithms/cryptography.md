# Cryptography

Reference for the `algorithmist` persona. This is where the persona's anti-novelty rule is absolute. **Never implement a cryptographic primitive or protocol, and never compose primitives into your own scheme.** Choose a vetted, high-level library and its misuse-resistant API, and use a standard protocol (TLS, Noise) rather than designing one. Recommendations age: the answers below reflect Latacora's *Cryptographic Right Answers: Post Quantum Edition* (2024, updated 2026) and OWASP's password storage guidance. Re-check both before relying on this page.

## When you see this → reach for

| Problem shape | Reach for | Not | Use |
|---|---|---|---|
| Encrypt data at rest | a KMS, or an AEAD with a random-nonce-safe construction | unauthenticated modes (ECB, CBC, raw CTR) | libsodium `secretbox` or XChaCha20-Poly1305; RustCrypto `chacha20poly1305` |
| AEAD where AES hardware and standards compliance matter | AES-256-GCM | random 96-bit nonces at high volume | RustCrypto `aes-gcm`, `ring`, OpenSSL/BoringSSL |
| Store user passwords | a slow, memory-hard password hash | SHA-*, MD5, any fast hash, even salted | Argon2id (`argon2` crate, libsodium `crypto_pwhash`), then scrypt, then bcrypt |
| Authenticate a message with a shared key | HMAC-SHA-256 (or BLAKE3 keyed mode) | `hash(key ‖ msg)`; HMAC-MD5/SHA-1 | RustCrypto `hmac` + `sha2`, `ring::hmac` |
| Compare MACs, tokens, password hashes | constant-time comparison | `==` on secret bytes | `subtle`, `ring`'s verify functions, libsodium `sodium_memcmp` |
| Sign so others can verify | Ed25519 (hybrid with ML-DSA for long-lived keys) | RSA PKCS#1 v1.5, DIY ECDSA | `ed25519-dalek`, `ring`, libsodium `crypto_sign` |
| Agree on a key over a network | don't: use TLS 1.3, or a Noise-based protocol | a custom handshake | `rustls`, OpenSSL/BoringSSL; Noise (as in WireGuard) |
| Raw key exchange inside an existing protocol | X25519 (hybrid X25519 + ML-KEM-768 for post-quantum) | finite-field DH you parameterize yourself | `x25519-dalek`, libsodium `crypto_kx` |
| Derive keys from a shared secret | HKDF | ad-hoc hashing of secrets | RustCrypto `hkdf`, `ring::hkdf` |
| Integrity or content addressing | SHA-256 / SHA-2, or BLAKE3 | MD5, SHA-1 | `sha2`, `blake3`; see [hashing-and-caching.md](./hashing-and-caching.md) |
| Random bytes for keys, tokens, nonces | the OS CSPRNG | seeded or userspace PRNGs, `rand()`, time-based seeds | `getrandom`, `rand`'s OS-backed RNG (`SysRng` as of rand 0.10), libsodium `randombytes` |
| Unguessable IDs (reset links, API keys, session IDs) | 256 random bits from the OS CSPRNG | UUIDs (RFC 9562: not security capabilities), hashes of predictable data | `getrandom`, then hex or base64url |
| Keep secrets out of memory dumps and logs | zeroize on drop; redacting `Debug` | printing configs, secrets in URLs | `zeroize`, `secrecy` |

## Reach-for notes

- **Prefer the highest-level API.** libsodium's `secretbox`, `crypto_box` and `crypto_pwhash` choose algorithms, nonce sizes and encodings for you. Reach for raw primitives (RustCrypto crates, `ring`) only inside a design reviewed by someone who does this for a living.
- **Nonces:** AES-GCM's 96-bit nonce must never repeat under a key. Reuse destroys both confidentiality and authenticity. libsodium advises capping how much data one key encrypts, and deriving nonces from counters ([docs](https://doc.libsodium.org/secret-key_cryptography/aead/aes-256-gcm)). XChaCha20-Poly1305's 192-bit nonce is safe to pick at random ([docs](https://doc.libsodium.org/secret-key_cryptography/aead/chacha20-poly1305/xchacha20-poly1305_construction)).
- **Password hashing parameters** (OWASP): Argon2id with at least 19 MiB of memory, 2 iterations and parallelism 1; else scrypt (N = 2^17, r = 8, p = 1); bcrypt only for legacy systems (cost ≥ 10; inputs truncated at 72 bytes); PBKDF2-HMAC-SHA-256 with ≥ 600,000 iterations where FIPS is required ([cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)).
- **Symmetric keys:** 256-bit. **Post-quantum:** NIST standardized ML-KEM and ML-DSA in 2024. Latacora recommends *hybrid* constructions (X25519 + ML-KEM-768, Ed25519 + ML-DSA-65 for long-lived keys), not pure post-quantum ([post](https://www.latacora.com/blog/post-quantum-cryptographic-right-answers/)).
- **TLS:** 1.3 by default; use `rustls` or the platform's OpenSSL/BoringSSL, or terminate at a managed load balancer. Don't write custom encrypted transports.
- **Non-cryptographic hashes** (SipHash, xxHash, FxHash) are for hash tables and checksums. Never use them where an attacker benefits from a collision or a forgery.

## Classic failure modes

- **Rolling your own:** a custom cipher mode, encrypt-then-hash "MAC", homemade token format or handshake.
- **ECB mode** (patterns show through) and **unauthenticated encryption** (malleable ciphertext).
- **Nonce reuse** with GCM or ChaCha20-Poly1305.
- **Timing leaks:** early-exit comparisons of MACs or tokens; secret-dependent branches.
- **Fast hashes for passwords**, including salted SHA-256.
- **MD5 or SHA-1 where security matters** — signatures, integrity against attackers, certificates.
- **Seeded PRNGs for secrets**, or seeds from time or PIDs.
- **Secrets in logs, URLs, error messages or crash dumps.**
- **Stale recommendations:** algorithm advice has a date on it. Re-check before each new design.

**Depth:** Ferguson, Schneier and Kohno, *Cryptography Engineering*; Aumasson, *Serious Cryptography* (2nd ed.); Latacora, [*Cryptographic Right Answers: Post Quantum Edition*](https://www.latacora.com/blog/post-quantum-cryptographic-right-answers/); [OWASP Cheat Sheet Series](https://cheatsheetseries.owasp.org/).
