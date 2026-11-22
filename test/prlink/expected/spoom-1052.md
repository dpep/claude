[`Client#send_raw`](https://github.com/Shopify/spoom/blob/c944a0e/lib/spoom/sorbet/lsp.rb#L30) frames each request as `Content-Length:N\r\n\r\n<body>\n` (lsp.rb#L31). That has no space after the colon, counts N in characters, and `puts` adds a newline after the body. The LSP spec requires `Content-Length: N\r\n\r\n<body>`, with N in bytes.

So a request with a non-ASCII character (say, a `workspace/symbol` query for `Café`) reports a length one byte short. The server cuts the body short, and the missing byte plus the stray newline end up at the front of the next message. A server that parses the header strictly rejects every request.

This writes the header with the space and `bytesize`, and uses [`IO#write`](https://docs.ruby-lang.org/en/4.0/IO.html#method-i-write) so nothing follows the body.

Tests: [`client_framing_test.rb`](https://github.com/dpep/spoom/blob/c28f521/test/spoom/sorbet/lsp/client_framing_test.rb) sends a non-ASCII request twice to a tiny Ruby server that accepts only spec-exact framing. It fails on `main`.
