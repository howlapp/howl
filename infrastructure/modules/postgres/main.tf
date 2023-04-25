resource "helm_release" "postgres" {
  name       = "postgres"
  repository = "https://charts.bitnami.com/bitnami"
  chart      = "postgresql"
  version    = "12.3.1"
  namespace  = "macrostack"
  values = [
    templatefile("${path.module}/values.yaml", {
      postgres_version  = var.postgres_version
      postgres_password = var.postgres_password
    })
  ]

}
