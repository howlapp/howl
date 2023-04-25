resource "kubernetes_manifest" "example-service" {
  manifest = yamldecode(file("${path.module}/service.yaml"))
}

resource "kubernetes_manifest" "example-service-deployment" {
  manifest = yamldecode(templatefile("${path.module}/deployment.yaml", {
    commit = var.commit
  }))
  depends_on = [
    kubernetes_service_account.example-service-account
  ]
}

resource "kubernetes_service_account" "example-service-account" {
  metadata {
    namespace = "macrostack"
    name      = "example-service"
  }
  automount_service_account_token = true
}
