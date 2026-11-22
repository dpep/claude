[`Client#close`](https://github.com/Shopify/spoom/blob/c944a0e/lib/spoom/sorbet/lsp.rb#L227) sends `shutdown` with `params: {}` (lsp.rb#L228). The LSP spec defines its params as `void`, and Sorbet rejects the request:

> Unable to deserialize LSP request: Expected field `RequestMessage.params` to have value of type `null`, but had value `{}`.

So `close` always raises [`Spoom::LSP::ResponseError`](https://github.com/Shopify/spoom/blob/c944a0e/lib/spoom/sorbet/lsp/errors.rb#L36).

[`Request`](https://github.com/Shopify/spoom/blob/c944a0e/lib/spoom/sorbet/lsp/base.rb#L34) now takes optional params (base.rb#L42). [`Message#as_json`](https://github.com/Shopify/spoom/blob/c944a0e/lib/spoom/sorbet/lsp/base.rb#L18) already drops nil fields, so `shutdown` goes out without a `params` field. [`rbi/spoom.rbi`](https://github.com/dpep/spoom/blob/355a0cf/rbi/spoom.rbi) is regenerated.

Tests: [`client_shutdown_test.rb`](https://github.com/dpep/spoom/blob/355a0cf/test/spoom/sorbet/lsp/client_shutdown_test.rb) closes a client connected to a tiny Ruby server that rejects `shutdown` with params. It fails on `main`. The test server also accepts Spoom's current framing, so this doesn't depend on #1052.

