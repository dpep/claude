# frozen_string_literal: true

# One test per finding from the senior and gold testers, against small
# synthetic repos (test/repos.rb). Each was seen failing on the old prlink
# (PRLINK_BIN=<old build> script/prlink-test.sh -n /Fixes/).

require_relative "repos"

class PrlinkFixesTest < Minitest::Test
  include PrlinkHelper

  BASE = "https://github.com/acme/widget/blob/"
  HEAD = "https://github.com/someone/widget/blob/"

  def run_body(body, dir: Repos.widget, args: [], env: {})
    Dir.mktmpdir do |d|
      path = File.join(d, "body.md")
      File.binwrite(path, body)
      out, err, st = Open3.capture3(ENV_VARS.merge(env), BIN, "--body", path, "-C", dir, *args)
      [out, err, st.exitstatus]
    end
  end

  # The --json document for a body, or flunks with what prlink printed.
  def json(body, **kw)
    out, err, = run_body(body, args: ["--json"], **kw)
    JSON.parse(out)
  rescue JSON::ParserError
    flunk "not JSON:\n#{out}\n#{err}"
  end

  def ref(doc, text)
    doc["refs"].find { |r| r["span"]["text"] == text } or flunk "no ref #{text} in #{doc["refs"].map { |r| r["span"]["text"] }}"
  end

  # --- wrong links ---

  def test_rust_trait_impls_are_ambiguous
    out, err, = run_body("`Thing#go` and `Thing#solo`\n")
    refute_match(/\[`Thing#go`\]/, out)
    assert_match(%r{\[`Thing#solo`\]\(#{BASE}\h+/src.rs#L7\)}, out)
    assert_match(/`Thing#go`: rq: Thing#go is declared 2 times.*\n.*src.rs#L4.*\n.*src.rs#L5/, err)
  end

  def test_plain_word_is_reported_not_linked
    doc = json("the `price` column, `price()`, `#price`, `fresh_start`\n")
    word = ref(doc, "`price`")
    assert_equal "ambiguous", word["status"]
    assert_nil word["chosen"]
    assert_equal ["lib/widget.rb"], word["candidates"].map { |c| c["path"] }
    %w[`price()` `#price` `fresh_start`].each { |t| assert_equal "linked", ref(doc, t)["status"], t }
  end

  def test_coincident_line_inside_a_hunk_is_ambiguous
    out, err, = run_body("closes at f.rb:3, blank f.rb:4, and f.rb:7\n", dir: Repos.five)
    assert_equal "closes at f.rb:3, blank f.rb:4, and f.rb:7\n", out
    assert_match(/f.rb:3: f.rb#L3 reads the same at base and head, but the PR changed or moved the code around it/, err)
    assert_match(/f.rb:7: .*\n.*#L7  "end"\n.*#L7  "end"/, err)
  end

  def test_ranges_link_whole_or_not_at_all
    body = "lib/widget/added.rb#L3-L5 and lib/widget/added.rb:3-5 and `lib/widget/added.rb#L3-L5`\n"
    out, = run_body(body)
    range = %r{#{HEAD}\h+/lib/widget/added.rb#L3-L5\)}
    assert_equal 3, out.scan(range).size, out
    refute_match(/\)-L?5/, out)
  end

  def test_range_on_changed_lines_is_ambiguous_with_both_ranges
    doc = json("lib/widget.rb#L10-L20\n")
    r = ref(doc, "lib/widget.rb#L10-L20")
    assert_equal "ambiguous", r["status"]
    assert_equal [[10, 20, "base"], [10, 20, "head"]], r["candidates"].map { |c| [c["line"], c["end_line"], c["side"]] }
  end

  def test_existing_link_checks_owner_and_whole_range
    sha = Repos.git(Repos.widget, "rev-parse", "--short=7", "origin/main")
    url = "#{BASE}#{sha}/lib/widget.rb"
    body = <<~MD
      A [`Widget::Item#render`](#{url}#L11) owner-wrong.
      F [`price`](#{url}#L18-L21) range holds the def.
      R [`price`](#{url}#L22-L24) range past the def.
      H [`Widget::Item#price`](#{url}?plain=1#L5) plain.
      C [`Widget::Base#build`](#{url}#L5#L9) doubled.
      I [`price`](https://github.com/acme/widget/blob/main/lib/widget.rb#L19) branch.
    MD
    doc = json(body)
    by = doc["refs"].map { |r| [r["span"]["text"][/\[(.*?)\]/, 1], r] }
    a = by[0][1]
    assert_equal "existing_wrong", a["status"]
    assert_equal "#{url}#L15", a["chosen"]["url"]
    assert_equal "existing_ok", by[1][1]["status"]
    assert_equal "existing_wrong", by[2][1]["status"]
    assert_nil by[2][1]["chosen"], "a range is never shrunk"
    assert_equal "#{url}?plain=1#L19", by[3][1]["chosen"]["url"]
    assert_equal "existing_wrong", by[4][1]["status"]
    assert_equal "existing_unpinned", by[5][1]["status"]
  end

  # --- mangled text ---

  # The senior tester's markdown repros, byte for byte.
  %w[md1 md1crlf s1 h1 bom].each do |name|
    define_method("test_markdown_#{name}") do
      input = File.binread(File.join(__dir__, "fixtures/md/#{name}.md"))
      out, err, = run_body(input)
      expected = File.join(__dir__, "expected/md/#{name}.md")
      File.binwrite(expected, out) if ENV["UPDATE_GOLDEN"]
      assert_equal File.binread(expected), out.b, "#{name} (UPDATE_GOLDEN=1)\n#{err}"
    end
  end

  def test_rerun_on_own_output_changes_nothing
    input = File.binread(File.join(__dir__, "fixtures/md/md1.md"))
    once, = run_body(input)
    twice, err, = run_body(once)
    assert_equal once, twice, err
  end

  # --- crashes ---

  def test_non_utf8_body_exits_2_without_a_trace
    _, err, st = run_body("ok `Widget::Added#fresh` caf\xE9\n".b)
    assert_equal 2, st
    assert_match(/not valid UTF-8 \(first bad byte at offset 28\)/, err)
    refute_match(/\.rb:\d+:in /, err)
  end

  def test_missing_tool_degrades
    out, err, st = run_body("`Widget::Added#fresh` and `lib/widget.rb`\n", env: { "PRLINK_TREKR" => "/nonexistent/trekr" })
    assert_equal 1, st
    refute_match(/\.rb:\d+:in |Errno/, err)
    assert_match(/unresolved \(tool missing\):\n  `Widget::Added#fresh`: trekr \(\/nonexistent\/trekr\) not found/, err)
    assert_includes out, "[`lib/widget.rb`](#{HEAD}"
    doc = json("`Widget::Added#fresh`\n", env: { "PRLINK_TREKR" => "/nonexistent/trekr" })
    assert_equal ["trekr (/nonexistent/trekr)"], doc["summary"]["tools_missing"]
    assert_equal "tool_missing", doc["refs"][0]["cause"]
  end

  def test_rq_runs_with_prlink_named_as_its_caller
    Dir.mktmpdir do |d|
      log = File.join(d, "caller")
      stub = File.join(d, "rq")
      File.write(stub, "#!/bin/sh\nprintf '%s\\n' \"$RQ_CALLER\" >> #{log}\necho '[]'\n")
      File.chmod(0o755, stub)
      run_body("`Thing#solo`\n", env: { "PRLINK_RQ" => stub })
      assert_equal ["prlink"], File.readlines(log, chomp: true).uniq
    end
  end

  # --- loud where it was quiet ---

  def test_unpushed_head_is_loud_and_write_refuses
    Dir.mktmpdir do |d|
      body = File.join(d, "b.md")
      File.write(body, "`Widget::Local#here`\n")
      args = ["--body", body, "-C", Repos.widget, "--head", "local"]
      doc = JSON.parse(Open3.capture3(ENV_VARS, BIN, *args, "--json")[0])
      assert_equal false, doc["head_pushed"]
      assert_equal 1, doc["summary"]["exit"]
      _, err, st = Open3.capture3(ENV_VARS, BIN, *args, "--write")
      assert_equal 1, st.exitstatus
      assert_match(/not written: head \h+ is on no remote/, err)
      assert_equal "`Widget::Local#here`\n", File.read(body)
      Open3.capture3(ENV_VARS, BIN, *args, "--write", "--allow-unpushed")
      assert_match(/\[`Widget::Local#here`\]/, File.read(body))
    end
  end

  def test_shallow_clone_says_to_deepen
    _, err, st = run_body("`Widget::Added#fresh`\n", dir: Repos.shallow, args: ["--base", "origin/main"])
    assert_equal 2, st
    assert_match(/no merge-base .* shallow clone: `git fetch --deepen=200`/, err)
    _, err, = run_body("`Widget::Added#fresh`\n", dir: Repos.shallow)
    assert_match(/warning: shallow clone: .*`git fetch --deepen=200`/, err)
  end

  def test_subdirectory_resolves_from_the_repo_root
    body = "`Widget::Added#fresh` and `lib/widget/added.rb`\n"
    from_root, = run_body(body)
    from_lib, err, = run_body(body, dir: File.join(Repos.widget, "lib/widget"))
    assert_equal from_root, from_lib, err
    assert_includes from_lib, "[`lib/widget/added.rb`]"
  end

  def test_paths_with_spaces_and_unicode
    out, = run_body("`dir with space/notes.rb:1` and `café.rb`\n")
    assert_match(%r{\]\(#{BASE}\h+/dir%20with%20space/notes.rb#L1\)}, out)
    assert_match(%r{\]\(#{BASE}\h+/caf%C3%A9.rb\)}, out)
  end

  def test_linked_report_is_deduped
    _, err, = run_body("#{"`Widget::Added#fresh` " * 5}\n")
    assert_match(/  `Widget::Added#fresh` -> \S+ \(x5\)\n/, err)
    assert_equal 1, err.scan("`Widget::Added#fresh` ->").size
  end

  def test_unresolved_splits_external_from_not_found
    doc = json("`vendor/sub/inner.rb` and `lib/nope.rb`\n")
    assert_equal %w[external vendor/sub], ref(doc, "`vendor/sub/inner.rb`").values_at("cause", "where")
    assert_equal "not_found", ref(doc, "`lib/nope.rb`")["cause"]
  end

  def test_report_names_base_and_head
    _, err, = run_body("x\n")
    assert_match(/^base \h{7} in acme\/widget: merge-base of origin\/main and HEAD\n/, err)
    assert_match(/^head \h{7} in someone\/widget: HEAD\n/, err)
  end

  def test_parenthesized_plain_paths_are_never_silent
    doc = json("see (lib/widget/added.rb:3) and (lib/widget.rb)\n")
    assert_equal "linked", ref(doc, "lib/widget/added.rb:3")["status"]
    plain = ref(doc, "lib/widget.rb")
    assert_equal "left_alone", plain["status"]
    assert_match(/backticks/, plain["reason"])
    assert_equal 1, plain["candidates"].size
  end

  def test_bare_names_outside_touched_files_are_reported
    doc = json("`untouched_helper` and `generic`\n")
    assert_equal %w[unresolved not_found], ref(doc, "`untouched_helper`").values_at("status", "cause")
    generic = ref(doc, "`generic`")
    assert_equal "left_alone", generic["status"]
    assert_match(/plain word/, generic["reason"])
  end

  # --- the read-only interface ---

  def test_json_shape
    doc = json("`Widget::Added#fresh` [`price`](https://github.com/acme/widget/blob/main/lib/widget.rb#L19)\n")
    assert_equal %w[base head head_pushed refs summary], doc.keys
    assert_equal %w[sha short repo from pushed], doc["base"].keys
    assert_equal %w[span shape query status cause reason where candidates chosen existing], doc["refs"][0].keys
    assert_equal %w[line col text], doc["refs"][0]["span"].keys
    assert_equal %w[url repo sha side path line end_line tool evidence owner], doc["refs"][0]["chosen"].keys
    assert_equal %w[url repo rev path line end_line], doc["refs"][1]["existing"].keys
    assert_equal %w[refs linked ambiguous unresolved left_alone existing_ok existing_wrong existing_stale existing_unpinned
                    tools_missing warnings exit], doc["summary"].keys
  end

  def test_ndjson_is_one_ref_per_line_then_the_summary
    out, = run_body("`Widget::Added#fresh` and `price`\n", args: ["--ndjson"])
    lines = out.lines.map { |l| JSON.parse(l) }
    assert_equal 3, lines.size
    assert_equal %w[`Widget::Added#fresh` `price`], lines[0..1].map { |r| r["span"]["text"] }
    assert_equal %w[base head head_pushed summary], lines.last.keys
  end

  def test_json_changes_nothing
    _, err, st = run_body("`Widget::Added#fresh`\n", args: ["--json", "--write"])
    assert_equal 2, st
    assert_match(/only report; drop --write/, err)
  end

  def test_explain_shows_every_candidate_and_stdin_is_a_body
    out, err, st = Open3.capture3(ENV_VARS, BIN, "-C", Repos.widget, "--explain", stdin_data: "`Thing#go`\n")
    assert_equal [1, "`Thing#go`\n"], [st.exitstatus, out]
    assert_match(/^`Thing#go` \(line 1, col 1\): owner_method, ambiguous\n  reason: .*\n  - rq head Thing \S+src.rs#L4\n      "impl A for Thing/, err)
  end

  def test_help_documents_every_flag_and_its_examples_parse
    out, _, st = Open3.capture3(BIN, "--help")
    assert_equal 0, st.exitstatus
    %w[--body --pr --pr-json -C --repo --base --head --json --ndjson --explain --write --allow-unpushed].each do |flag|
      assert_match(/^\s+(-\w, )?#{flag}\b.* {2,}\S/, out, "#{flag} has no description")
    end
    assert_match(/^Exit codes:\n  0 .*\n  1 .*\n(.*\n)*?  2 /, out)
    assert_match(/HEAD~/, out)
    load BIN unless defined?(Prlink::CLI)
    examples = out[/^Examples:\n((?:  prlink.*\n)+)/, 1].lines
    refute_empty examples
    examples.each { |l| Prlink::CLI.parse(l.split.drop(1)) }
  end
end
