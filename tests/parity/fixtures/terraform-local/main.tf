module "local_mod" {
  source = "./modules/local"
}

resource "null_resource" "example" {
  triggers = {
    always_run = timestamp()
  }
}
