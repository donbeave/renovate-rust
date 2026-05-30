defmodule Example.MixProject do
  use Mix.Project

  def project do
    [
      app: :example,
      version: "1.0.0",
      deps: deps()
    ]
  end

  defp deps do
    [
      {:local_pkg, path: "../local_pkg"},
      {:git_pkg, git: "https://github.com/user/git_pkg.git"},
      {:noversion}
    ]
  end
end
