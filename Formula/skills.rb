class Skills < Formula
  desc "A CLI for managing skills for AI coding assistants"
  homepage "https://github.com/skillsmd/skillscli"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/skillsmd/skillscli/releases/download/v0.1.0/skills-macos-aarch64.tar.gz"
      sha256 "02314e9e5ecf55ab7f75f984cc956545e0cf3ac0bb54ec6f722b5ca12e2f4797"
    else
      url "https://github.com/skillsmd/skillscli/releases/download/v0.1.0/skills-macos-x86_64.tar.gz"
      sha256 "02314e9e5ecf55ab7f75f984cc956545e0cf3ac0bb54ec6f722b5ca12e2f4797"
    end
  end

  on_linux do
    url "https://github.com/skillsmd/skillscli/releases/download/v0.1.0/skills-linux-x86_64.tar.gz"
    sha256 "02314e9e5ecf55ab7f75f984cc956545e0cf3ac0bb54ec6f722b5ca12e2f4797"
  end

  def install
    bin.install "skills"
  end

  test do
    system "#{bin}/skills", "--version"
    system "#{bin}/skills", "--help"
  end
end
