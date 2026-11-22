# frozen_string_literal: true

require "fileutils"
require_relative "helper"

# Small synthetic repos for the fix tests, after the senior tester's repros.
# Commits are dated and authored fixed, so their shas are stable and prlink's
# snapshot cache stays warm between runs. Bump VERSION when a builder changes.
module Repos
  VERSION = 1
  DIR = File.join(PrlinkHelper::STATE, "repos-v#{VERSION}")
  GIT_ENV = {
    "GIT_DIR" => nil, "GIT_WORK_TREE" => nil, "GIT_INDEX_FILE" => nil,
    "GIT_AUTHOR_NAME" => "t", "GIT_AUTHOR_EMAIL" => "t@t", "GIT_COMMITTER_NAME" => "t", "GIT_COMMITTER_EMAIL" => "t@t",
    "GIT_AUTHOR_DATE" => "2026-01-01T00:00:00Z", "GIT_COMMITTER_DATE" => "2026-01-01T00:00:00Z",
    "GIT_CONFIG_GLOBAL" => File::NULL, "GIT_CONFIG_NOSYSTEM" => "1"
  }.freeze

  module_function

  def git(dir, *args)
    out, err, st = Open3.capture3(GIT_ENV, "git", "-C", dir, *args)
    raise "git #{args.join(" ")}: #{err}" unless st.success?

    out.strip
  end

  def write(dir, files)
    files.each do |path, body|
      full = File.join(dir, path)
      FileUtils.mkdir_p(File.dirname(full))
      File.write(full, body)
    end
  end

  def commit(dir, msg)
    git(dir, "add", "-A")
    git(dir, "commit", "-q", "-m", msg)
    git(dir, "rev-parse", "HEAD")
  end

  # Builds once into DIR/name (atomically), then reuses it.
  def build(name)
    dir = File.join(DIR, name)
    return dir if File.directory?(dir)

    tmp = "#{dir}.tmp#{Process.pid}"
    FileUtils.rm_rf(tmp)
    FileUtils.mkdir_p(tmp)
    git(tmp, "init", "-q", "-b", "main")
    yield tmp
    File.rename(tmp, dir)
    dir
  end

  WIDGET = <<~RB
    module Widget
      VERSION = "1"

      class Base
        def build(x)
          x + 1
        end

        def render
          "r"
        end
      end

      class Item < Base
        def render
          "item"
        end

        def price
          3
        end

        def self.make
          new
        end
      end

      def self.config
        {}
      end

      Config = Struct.new(:a)
    end

    class Item
      def reopened
        1
      end
    end
  RB

  # main: the base. feature: the head, pushed to the fork. local: one commit
  # past feature that no remote has.
  def widget
    build("widget") do |d|
      git(d, "remote", "add", "origin", "git@github.com:acme/widget.git")
      git(d, "remote", "add", "fork", "git@github.com:someone/widget.git")
      write(d, {
              "lib/widget.rb" => WIDGET,
              "lib/widget/old.rb" => "module Widget\n  class Old\n    def legacy\n      1\n    end\n\n    def keep\n      2\n    end\n  end\nend\n",
              "lib/untouched.rb" => "module Untouched\n  def self.untouched_helper\n    1\n  end\nend\n",
              "dir with space/notes.rb" => "line1\nline2\n",
              "café.rb" => "def cafe; end\n",
              "src.rs" => <<~RS
                pub struct Thing;
                pub trait A { fn go(&self); }
                pub trait B { fn go(&self); }
                impl A for Thing { fn go(&self) {} }
                impl B for Thing { fn go(&self) {} }
                impl Thing {
                    pub fn solo(&self) {}
                }
                pub fn generic<T: Clone>(x: T) -> T { x }
              RS
            })
      base = commit(d, "base")
      # A submodule is a gitlink in the tree; no repo behind it is needed.
      git(d, "update-index", "--add", "--cacheinfo", "160000,#{base},vendor/sub")
      git(d, "commit", "-q", "-m", "submodule")
      base = git(d, "rev-parse", "HEAD")
      git(d, "update-ref", "refs/remotes/origin/main", base)
      git(d, "symbolic-ref", "refs/remotes/origin/HEAD", "refs/remotes/origin/main")
      git(d, "checkout", "-q", "-b", "feature")
      write(d, {
              "lib/widget.rb" => "# header\n# header2\n#{WIDGET.sub(/^      3$/, "      4")}",
              "lib/widget/added.rb" => "module Widget\n  class Added\n    def fresh\n      1\n    end\n\n    def fresh_start\n      2\n    end\n  end\nend\n"
            })
      git(d, "mv", "lib/widget/old.rb", "lib/widget/older.rb")
      write(d, { "lib/widget/older.rb" => "module Widget\n  class Old\n    def keep\n      2\n    end\n  end\nend\n" })
      head = commit(d, "feature")
      git(d, "update-ref", "refs/remotes/fork/feature", head)
      git(d, "checkout", "-q", "-b", "local")
      write(d, { "lib/widget/local.rb" => "module Widget\n  class Local\n    def here\n      1\n    end\n  end\nend\n" })
      commit(d, "local")
      git(d, "checkout", "-q", "feature")
    end
  end

  # A new method lands above old ones, so line numbers shift and `end`s coincide.
  def five
    build("five") do |d|
      git(d, "remote", "add", "origin", "git@github.com:acme/five.git")
      body = "def a\n  x\nend\n\ndef b\n  y\nend\n"
      write(d, { "f.rb" => body })
      base = commit(d, "base")
      git(d, "update-ref", "refs/remotes/origin/main", base)
      git(d, "symbolic-ref", "refs/remotes/origin/HEAD", "refs/remotes/origin/main")
      git(d, "checkout", "-q", "-b", "p")
      write(d, { "f.rb" => "def n\n  q\nend\n\n#{body}" })
      git(d, "update-ref", "refs/remotes/origin/p", commit(d, "p"))
    end
  end

  # A depth-1 clone of widget's feature branch: no base to compare against.
  def shallow
    src = widget
    dir = File.join(DIR, "shallow")
    return dir if File.directory?(dir)

    _, err, st = Open3.capture3(GIT_ENV, "git", "clone", "-q", "--depth", "1", "--no-single-branch", "--branch", "feature", "file://#{src}", dir)
    raise "clone: #{err}" unless st.success?

    git(dir, "remote", "set-url", "origin", "git@github.com:acme/widget.git")
    dir
  end
end
