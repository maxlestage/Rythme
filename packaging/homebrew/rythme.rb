# Homebrew formula of Rythme (template). For each release, the release
# workflow fills in the version and the checksums of the archives with
# packaging/render.sh and attaches the result, rythme.rb, to the release.
# See packaging/README.md to publish it in a tap.
class Rythme < Formula
  desc "Simple, purely functional programming language with help everywhere"
  homepage "https://maxlestage.github.io/Rythme/"
  version "@VERSION@"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/maxlestage/Rythme/releases/download/v@VERSION@/rythme-macos-arm64.tar.gz"
      sha256 "@SHA256:rythme-macos-arm64.tar.gz@"
    end
    on_intel do
      url "https://github.com/maxlestage/Rythme/releases/download/v@VERSION@/rythme-macos-x86_64.tar.gz"
      sha256 "@SHA256:rythme-macos-x86_64.tar.gz@"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/maxlestage/Rythme/releases/download/v@VERSION@/rythme-linux-arm64.tar.gz"
      sha256 "@SHA256:rythme-linux-arm64.tar.gz@"
    end
    on_intel do
      url "https://github.com/maxlestage/Rythme/releases/download/v@VERSION@/rythme-linux-x86_64.tar.gz"
      sha256 "@SHA256:rythme-linux-x86_64.tar.gz@"
    end
  end

  def install
    bin.install "rythme"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/rythme --version")
    assert_equal "3\n", shell_output("#{bin}/rythme -e 'print(1 + 2)'")
  end
end
