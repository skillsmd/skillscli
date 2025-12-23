class Skills < Formula
  desc "A CLI for managing skills for AI coding assistants"
  homepage "https://github.com/skillsmd/skillscli"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/skillsmd/skillscli/releases/download/v0.1.0/skills-macos-aarch64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/skillsmd/skillscli/releases/download/v0.1.0/skills-macos-x86_64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  end

  on_linux do
    url "https://github.com/skillsmd/skillscli/releases/download/v0.1.0/skills-linux-x86_64.tar.gz"
    sha256 "REPLACE_WITH_ACTUAL_SHA256"
  end

  def install
    bin.install "skills"
  end

  test do
    system "#{bin}/skills", "--version"
    system "#{bin}/skills", "--help"
  end
end
