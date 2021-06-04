$formula_name = "{{ name }}"

class {{ name_pascal_case }} < Formula
  version $version_mac
  desc "{{ description }}"
  homepage "{{ homepage }}"
  head "{{ repository }}"

  if OS.mac?
    url "{{ repository_url }}/releases/download/#{$version_mac}/#{$formula_name}-#{$version_mac}-x86_64-mac.zip"
    sha256 $sha_mac
  elsif OS.linux?
    url "{{ repository_url }}/releases/download/#{$version_linux}/#{$formula_name}-#{$version_linux}-x86_64-linux.zip"
    sha256 $sha_linux
  end

  def install
    bin.install "{{ name }}"
  end

  test do
    system "#{bin}/{{ name }}", '--version'
  end
end
