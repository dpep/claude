# frozen_string_literal: true

# Gold set: PR descriptions linked by hand. Each body is stripped to its link
# text, re-linked by prlink against a clone at the PR's own shas, and compared
# with the original. Every miss, different target and extra link must be
# explained in gold.yml. A link prlink gets wrong fails the target check unless
# gold.yml lists it under `wrong`: a known failure, its URL pinned by
# expected/*.md, for prlink's fix lane.
#
# Offline once the clones exist (see PrlinkHelper#clone_for).
# UPDATE_GOLDEN=1 rewrites expected/*.md.

require_relative "helper"

class PrlinkGoldTest < Minitest::Test
  include PrlinkHelper

  # [[text, occurrence], url]
  def keyed(pairs)
    seen = Hash.new(0)
    pairs.map { |t, u| [[t, seen[t] += 1], u] }
  end

  # Independent of prlink: the link's line holds the name its text cites.
  def check_target(text, url, clone)
    if url.start_with?("https://docs.ruby-lang.org/en/")
      return assert_match(%r{\Ahttps://docs\.ruby-lang\.org/en/\d+\.\d+/[\w/]+\.html(#method-[ci]-[\w-]+)?\z}, url)
    end

    m = url.match(%r{\Ahttps://github\.com/[^/]+/[^/]+/blob/([0-9a-f]{7})/([^#]+)(?:#L(\d+))?\z})
    assert m, "#{text}: unexpected URL #{url}"
    src, _, st = Open3.capture3("git", "-C", clone, "show", "#{m[1]}:#{m[2]}")
    assert st.success?, "#{text}: #{m[2]} missing at #{m[1]}"
    code = text[/\A`(.+)`\z/, 1] || text
    cited = code.sub(/#L\d+\z|:\d+\z/, "").delete_suffix("()")
    if cited.include?("/") || m[2].end_with?(cited)
      assert m[2].end_with?(cited), "#{text}: #{url} is not that file"
      assert_operator m[3].to_i, :<=, src.lines.size if m[3]
    else
      name = cited.split(/#|\.|::/).last
      line = src.lines[m[3].to_i - 1].to_s
      assert_match(/(?<![\w])#{Regexp.escape(name)}(?![\w])/, line, "#{text}: #{url} line does not mention #{name}")
    end
  end

  GOLD["cases"].each do |name, case_|
    define_method("test_#{name.tr("-", "_")}") do
      meta = fixture(name)
      original = meta["body"]
      out, err, status = prlink(name, meta.merge("body" => strip(original)))
      expected_file = File.join(__dir__, "expected", "#{name}.md")
      if ENV["UPDATE_GOLDEN"]
        File.write(expected_file, out)
      else
        assert_equal File.read(expected_file), out, "output drifted from expected/#{name}.md (UPDATE_GOLDEN=1)"
      end

      wrong = (case_["wrong"] || {}).keys
      gold = keyed(links(original)).to_h
      got = keyed(links(out)).to_h
      score = Hash.new { |h, k| h[k] = [] }
      gold.each do |key, url|
        if !got.key?(key) then score[:missed] << key.first
        elsif got[key] == url then score[:exact] << key.first
        elsif wrong.include?(key.first) then score[:wrong] << key.first
        else score[:different] << key.first
        end
      end
      got.each_key do |key|
        next if gold.key?(key)

        score[wrong.include?(key.first) ? :wrong : :extra] << key.first
      end
      clone = clone_for(name)
      got.each { |(text, _), url| check_target(text, url, clone) unless wrong.include?(text) }

      detail = (gold.keys | got.keys).reject { |k| gold[k] == got[k] }
                                     .map { |k| "  #{k.first}\n    gold: #{gold[k]}\n    got:  #{got[k]}" }.join("\n")
      %w[different missed extra wrong].each do |kind|
        explained = (case_[kind] || {}).keys
        assert_equal explained.sort, score[kind.to_sym].uniq.sort,
                     "#{name}: #{kind} links not as explained in gold.yml\n#{detail}\n#{err}"
      end
      assert_equal case_["exit"], status, "#{name}: exit status\n#{err}"
      warn format("%-22s exact %2d  different %2d  missed %2d  extra %2d  wrong %d",
                  name, *%i[exact different missed extra wrong].map { |k| score[k].size })
    end
  end
end
