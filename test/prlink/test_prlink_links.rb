# frozen_string_literal: true

# Existing links are verified at their own sha and never removed; what is
# not prose (fences, blockquotes, link text) is never rewritten.

require_relative "helper"

class PrlinkLinksTest < Minitest::Test
  include PrlinkHelper

  SEND = "https://github.com/Shopify/spoom/blob/c944a0e/lib/spoom/sorbet/lsp.rb"

  def test_keeps_every_hand_made_link
    GOLD["cases"].each_key do |name|
      body = fixture(name)["body"]
      out, err, = prlink(name, fixture(name))
      urls = links(out).map(&:last)
      fixed = err[/^fixed existing link:\n((?:  .*\n)+)/, 1].to_s
      links(body).each do |text, url|
        next if fixed.include?("#{text}: ")

        assert_includes urls, url, "#{name}: #{text} lost its link"
      end
    end
  end

  def test_fixes_a_line_outside_the_definition
    meta = fixture("spoom-1053")
    meta["body"] = meta["body"].sub("#{SEND}#L35", "#{SEND}#L30")
    out, err, = prlink("spoom-1053", meta)
    assert_includes out, "[`Client#send`](#{SEND}#L35)"
    assert_match(/fixed existing link:\n  `Client#send`: line 30 .* -> 35/, err)
  end

  # Line 36 is inside Client#send: a citation of the call it makes, not a wrong line.
  def test_keeps_a_line_inside_the_definition
    meta = fixture("spoom-1053")
    meta["body"] = meta["body"].sub("#{SEND}#L35", "#{SEND}#L36")
    out, err, = prlink("spoom-1053", meta)
    assert_includes out, "[`Client#send`](#{SEND}#L36)"
    refute_match(/fixed existing link/, err)
  end

  # The hand-made range L164-L169 holds the def (L166): kept whole, never shrunk.
  def test_range_holding_the_definition_is_kept
    model = "https://github.com/dpep/polyid/blob/fa6547d/lib/polyid/model.rb"
    out, err, = prlink("polyid-72", fixture("polyid-72"))
    assert_includes out, "[`polyid_after_commit`](#{model}#L164-L169)"
    refute_match(/fixed existing link/, err)
  end

  def test_leaves_fences_blockquotes_and_link_text_alone
    meta = fixture("spoom-1053")
    meta["body"] = <<~MD
      `Client#send`

      ```ruby
      `Client#send`
      ```

      > `Client#send`

      [see `Client#send` here](https://example.com)
    MD
    out, = prlink("spoom-1053", meta)
    assert out.start_with?("[`Client#send`](#{SEND}#L35)\n"), out
    assert_includes out, "```ruby\n`Client#send`\n```"
    assert_includes out, "> `Client#send`\n"
    assert_includes out, "[see `Client#send` here](https://example.com)"
  end
end

class PrlinkCliTest < Minitest::Test
  include PrlinkHelper

  def test_body_file_links_against_local_shas
    clone = clone_for("spoom-1052")
    # The head sits on a fork's branch, as in a contributor's clone; no fetch needed.
    unless system("git", "-C", clone, "remote", "get-url", "fork", out: File::NULL, err: File::NULL)
      system("git", "-C", clone, "remote", "add", "fork", "https://github.com/dpep/spoom.git", exception: true)
    end
    system("git", "-C", clone, "update-ref", "refs/remotes/fork/fix-lsp-framing", "c28f521", exception: true)
    Dir.mktmpdir do |d|
      body = File.join(d, "body.md")
      File.write(body, "`Client#send_raw` and `client_framing_test.rb`\n")
      out, err, st = Open3.capture3(ENV_VARS, BIN, "--repo", "Shopify/spoom", "--body", body,
                                    "--base", "c944a0e", "--head", "c28f521", "-C", clone)
      assert_equal 0, st.exitstatus, err
      assert_includes out, "[`Client#send_raw`](https://github.com/Shopify/spoom/blob/c944a0e/lib/spoom/sorbet/lsp.rb#L30)"
      # c28f521 is only on the fork remote, so the head repo is the fork.
      assert_includes out, "[`client_framing_test.rb`](https://github.com/dpep/spoom/blob/c28f521/"
    end
  end

  def test_usage_error_exits_2
    _, err, st = Open3.capture3(ENV_VARS, BIN, "--pr", "1", "--body", "x.md")
    assert_equal 2, st.exitstatus
    assert_match(/usage: prlink/, err)
  end
end
