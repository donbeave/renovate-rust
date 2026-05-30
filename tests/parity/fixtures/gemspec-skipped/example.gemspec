Gem::Specification.new do |spec|
  spec.name = "example"
  spec.version = "1.0.0"
  spec.add_dependency "local_gem", path: "../local_gem"
  spec.add_dependency "git_gem", git: "https://github.com/user/git_gem.git"
  spec.add_dependency "noversion"
end
