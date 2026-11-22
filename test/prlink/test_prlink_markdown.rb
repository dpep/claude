# frozen_string_literal: true

# Prlink::Markdown.scan: which parts of a body a ref may be found in. Code
# spans and inline links come back as sites; prose comes back as text sites;
# everything rendered verbatim or owned by markup comes back as nothing.

require_relative "helper"
load PrlinkHelper::BIN unless defined?(Prlink::Markdown)

class PrlinkMarkdownTest < Minitest::Test
  # [[:code, content] | [:link, text, url] | [:text, chars], ...]
  def scan(md)
    Prlink::Markdown.scan(md).map do |s|
      case s.kind
      when :code then [:code, s.code]
      when :link then [:link, s.text, s.url]
      else [:text, md[s.start...s.stop]]
      end
    end
  end

  def codes(md)
    scan(md).select { |k, _| k == :code }.map(&:last)
  end

  def prose(md)
    scan(md).select { |k, _| k == :text }.map(&:last).join("|")
  end

  def test_code_spans_of_any_backtick_count
    assert_equal ["a", "`b`", "c`d", "x"], codes("`a` `` `b` `` ``` c`d ``` `x`\n")
  end

  def test_unmatched_backticks_are_text
    assert_equal ["fresh", "tick and"], codes("Also `fresh` then a lone ` tick and `Widget::Added`.\n")
  end

  def test_escaped_backticks_are_not_code
    assert_empty codes("Escaped \\`Widget::Added#fresh\\` backticks.\n")
  end

  def test_a_code_span_crosses_lines_in_a_paragraph
    assert_equal ["a b"], codes("x `a\nb` y\n")
  end

  def test_longer_fence_holds_a_shorter_one
    md = "````md\n```\n`inner`\n```\nafter `inside`\n````\n`out`\n"
    assert_equal ["out"], codes(md)
  end

  def test_fence_info_with_a_backtick_is_not_a_fence
    assert_equal %w[fence x], codes("Inline ```fence``` line `x` then.\n")
  end

  def test_tilde_fence
    assert_equal ["out"], codes("~~~\n`in`\n~~~\n`out`\n")
  end

  def test_indented_code_after_a_blank_line
    assert_equal ["out"], codes("para\n\n    `in`\n\n`out`\n")
  end

  def test_indented_list_continuation_is_prose
    assert_equal %w[a b c], codes("- top `a`\n  - nested `b`\n    1. deep `c`\n")
    assert_equal ["a", "b"], codes("1. item `a`\n\n    more `b`\n")
  end

  def test_html_block_runs_to_a_blank_line
    md = "<details>\n<summary>Sum `in`</summary>\n\nafter `out`\n\n</details>\n"
    assert_equal ["out"], codes(md)
  end

  def test_pre_and_comment_blocks
    assert_equal ["out"], codes("<pre>\n`in`\n</pre>\n<!--\n`in`\n-->\n`out`\n")
  end

  def test_inline_code_tags_and_attributes
    md = %(inside <code>lib/a.rb:3</code> <a href="lib/a.rb:3">x</a> <img src="lib/a.rb#L3"> tail\n)
    assert_equal "inside | |x| | tail", prose(md)
  end

  def test_autolinks_and_bare_urls
    md = "Autolink <https://example.com/a.rb:3> and https://example.com/v?f=a.rb:3&x=1 bare.\n"
    assert_equal "Autolink | and | bare.", prose(md)
  end

  def test_inline_links_and_images
    md = "[`A`](https://x.test/a) and ![`B`](https://x.test/b.png) and [t](<https://x.test/c d>)\n"
    assert_equal [[:link, "`A`", "https://x.test/a"], [:link, "t", "https://x.test/c d"]], scan(md).reject { |k, _| k == :text }
  end

  def test_reference_links_definitions_and_footnotes
    md = "[text][1] and [`Widget`][1] and [1]: stays, ref[^1].\n\n[1]: https://example.com/a_b_(c)\n[^1]: note `n`.\n"
    assert_equal ["n"], codes(md)
    assert_equal " and | and |: stays, ref|.|: note |.", prose(md)
  end

  def test_a_definition_line_after_a_paragraph_is_left_alone
    assert_equal "para", prose("para\n[ref]: lib/a.rb:3\n")
  end

  def test_blockquote_and_its_lazy_continuation
    assert_equal ["out"], codes("> `in`\nlazy `in`\n\n`out`\n")
  end

  def test_brackets_inside_a_code_span_are_not_a_link
    assert_equal ["a[i](x)", "B"], codes("Use `a[i](x)` then `B` end.\n")
  end

  def test_crlf
    assert_equal %w[a b], codes("`a`\r\n```\r\n`in`\r\n```\r\n`b`\r\n")
  end
end
