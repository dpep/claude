# frozen_string_literal: true

require "minitest/autorun"
require "json"
require "yaml"
require "open3"
require "tmpdir"

module PrlinkHelper
  ROOT = File.expand_path("../..", __dir__)
  # PRLINK_BIN runs the suite against another build (an old one, to see a new test fail).
  BIN = ENV["PRLINK_BIN"] || File.join(ROOT, "plugins/code/bin/prlink")
  GOLD = YAML.load_file(File.join(__dir__, "gold.yml"))
  LINK = /\[((?:[^\[\]]|\[[^\]]*\])*)\]\(([^()\s]+)\)/.freeze
  # Snapshots and indexes persist here between runs; isolated from the user's.
  STATE = File.join(Dir.tmpdir, "prlink-test")
  ENV_VARS = {
    "PRLINK_CACHE" => File.join(STATE, "snapshots"),
    "TREKR_DB" => File.join(STATE, "trekr.db"),
    "TREKR_USAGE" => "off",
    "RQ_DB" => File.join(STATE, "rq.db"),
    "PRLINK_TREKR" => ENV.fetch("PRLINK_TREKR", "trekr")
  }.freeze

  # Each case's repo is cloned here on first use; offline after that.
  REPOS = File.join(Dir.tmpdir, "prlink-gold")

  def fixture(name)
    JSON.parse(File.read(File.join(__dir__, "fixtures", "#{name}.json")))
  end

  # The case's local clone, holding the fixture's base and head shas.
  def clone_for(name)
    c = GOLD["cases"].fetch(name)
    dir = File.join(REPOS, c["repo"].split("/").last)
    meta = fixture(name)
    git = ->(*a) { system("git", "-C", dir, *a, out: File::NULL, err: File::NULL) }
    system("git", "clone", "-q", "https://github.com/#{c["repo"]}.git", dir, exception: true) unless File.directory?(dir)
    # A merged PR's head is often gone from every branch; GitHub keeps it under refs/pull.
    { meta["headRefOid"] => "refs/pull/#{c["pr"]}/head", meta["baseRefOid"] => meta["baseRefOid"] }.each do |sha, ref|
      next if git.call("cat-file", "-e", "#{sha}^{commit}")

      git.call("fetch", "-q", "origin", ref) or raise "cannot fetch #{ref} into #{dir}"
    end
    dir
  end

  def links(body)
    body.scan(LINK)
  end

  def strip(body)
    body.gsub(LINK) { Regexp.last_match(1) }
  end

  # Runs prlink over `meta` (a `gh pr view` JSON) as the gold case `name` sets it up.
  # Returns [stdout, stderr, exit status].
  def prlink(name, meta)
    c = GOLD["cases"].fetch(name)
    Dir.mktmpdir do |d|
      path = File.join(d, "pr.json")
      File.write(path, JSON.generate(meta))
      out, err, st = Open3.capture3(ENV_VARS, BIN, "--repo", c["repo"], "--pr-json", path,
                                    "-C", clone_for(name))
      [out, err, st.exitstatus]
    end
  end
end
