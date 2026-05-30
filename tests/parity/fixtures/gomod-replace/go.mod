module example.com/test

go 1.22

require (
	github.com/pkg/errors v0.8.0
)

replace github.com/pkg/errors => ../local-pkg
