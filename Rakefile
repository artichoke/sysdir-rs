# frozen_string_literal: true

require 'open-uri'
require 'shellwords'
require 'bundler/audit/task'
require 'rubocop/rake_task'

task default: %i[format lint]

desc 'Lint sources'
task lint: %i[lint:clippy lint:rubocop:autocorrect]

namespace :lint do
  RuboCop::RakeTask.new(:rubocop)

  desc 'Lint Rust sources with Clippy'
  task :clippy do
    sh 'cargo clippy --workspace --all-features --all-targets'
  end

  desc 'Lint Rust sources with Clippy restriction pass (unenforced lints)'
  task :'clippy:restriction' do
    lints = [
      'clippy::dbg_macro',
      'clippy::get_unwrap',
      'clippy::indexing_slicing',
      'clippy::panic',
      'clippy::print_stdout',
      'clippy::expect_used',
      'clippy::unwrap_used',
      'clippy::todo',
      'clippy::unimplemented',
      'clippy::unreachable'
    ]
    command = ['cargo', 'clippy', '--'] + lints.flat_map { |lint| ['-W', lint] }
    sh command.shelljoin
  end
end

desc 'Format sources'
task format: %i[format:rust format:text format:c]

namespace :format do
  desc 'Format Rust sources with rustfmt'
  task :rust do
    sh 'cargo fmt -- --color=auto'
  end

  desc 'Format text, YAML, and Markdown sources with prettier'
  task :text do
    sh 'npm run fmt'
  end

  desc 'Format .c and .h sources with clang-format'
  task :c do
    sh 'npm run fmt:c'
  end
end

desc 'Format sources'
task fmt: %i[fmt:rust fmt:text fmt:c]

namespace :fmt do
  desc 'Format Rust sources with rustfmt'
  task :rust do
    sh 'cargo fmt -- --color=auto'
  end

  desc 'Format text, YAML, and Markdown sources with prettier'
  task :text do
    sh 'npm run fmt'
  end

  desc 'Format .c and .h sources with clang-format'
  task :c do
    sh 'npm run fmt:c'
  end
end

desc 'Build Rust workspace'
task :build do
  sh 'cargo build --workspace'
end

desc 'Generate Rust API documentation'
task :doc do
  ENV['RUSTDOCFLAGS'] = '-D warnings -D rustdoc::broken_intra_doc_links --cfg docsrs'
  sh 'rustup run --install nightly cargo doc --workspace'
end

desc 'Generate Rust API documentation and open it in a web browser'
task :'doc:open' do
  ENV['RUSTDOCFLAGS'] = '-D warnings -D rustdoc::broken_intra_doc_links --cfg docsrs'
  sh 'rustup run --install nightly cargo doc --workspace --open'
end

desc 'Run sysdir unit tests'
task :test do
  sh 'cargo test --workspace'
end

Bundler::Audit::Task.new

namespace :release do
  link_check_files = FileList.new('**/*.md') do |f|
    f.exclude('node_modules/**/*')
    f.exclude('**/target/**/*')
    f.exclude('**/vendor/*/**/*')
    f.include('*.md')
    f.include('**/vendor/*.md')
  end

  link_check_files.sort.uniq.each do |markdown|
    desc 'Check for broken links in markdown files'
    task markdown_link_check: markdown do
      command = ['npx', 'markdown-link-check', '--config', '.github/markdown-link-check.json', markdown]
      sh command.shelljoin
      sleep(rand(1..5))
    end
  end
end

desc 'Generate native bindings with Rust bindgen'
task :bindgen do
  bindgen = %w[
    bindgen --use-core
    --allowlist-function sysdir.*
    --allowlist-type sysdir.*
    --blocklist-type sysdir_search_path_enumeration_state
    --allowlist-var PATH_MAX
    --default-enum-style rust_non_exhaustive
    --constified-enum sysdir_search_path_domain_mask_t
    --no-prepend-enum-name
    cext/sysdir.h
  ]
  bindgen_io = IO.popen(bindgen)
  File.open('src/sys.rs', 'w') do |f|
    f.puts <<~HEADER
      // @generated
      //
      // src/sys.rs
      //
      // Copyright (c) 2023 Ryan Lopopolo <rjl@hyperbo.la>
      //
      // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE> or
      // <http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT>
      // or <http://opensource.org/licenses/MIT>, at your option. All files in the
      // project carrying such notice may not be copied, modified, or distributed
      // except according to those terms.
    HEADER
    f.puts ''

    IO.copy_stream(bindgen_io, f)

    f.puts ''
    f.puts <<~NEWTYPE
      /// Opaque type for holding sysdir enumeration state.
      #[repr(transparent)]
      #[derive(Debug)]
      #[allow(missing_copy_implementations)]
      pub struct sysdir_search_path_enumeration_state(::core::ffi::c_uint);

      impl PartialEq<::core::ffi::c_uint> for sysdir_search_path_enumeration_state {
          fn eq(&self, other: &::core::ffi::c_uint) -> bool {
              self.0 == *other
          }
      }

      impl sysdir_search_path_enumeration_state {
          /// Return true if the state indicates the enumeration is finished.
          #[must_use]
          pub fn is_finished(&self) -> bool {
              self.0 == 0
          }
      }
    NEWTYPE
  end
end

desc 'Extract sysdir(3) man page'
task :manpage do
  IO.popen(%w[man sysdir]) do |man_io|
    IO.popen(%w[col -bx], 'r+') do |col_io|
      IO.copy_stream(man_io, col_io)

      man_io.close_read
      col_io.close_write

      IO.copy_stream(col_io, 'sysdir.3')
    end
  end
end
