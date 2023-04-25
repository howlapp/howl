resource "kubernetes_manifest" "gateway-service" {
  manifest = yamldecode(file("${path.module}/service.yaml"))
}

resource "kubernetes_manifest" "gateway-deployment" {
  manifest = yamldecode(templatefile("${path.module}/deployment.yaml", {
    commit = var.commit
  }))
}

resource "kubernetes_manifest" "gateway-service-defaults" {
  manifest = yamldecode(file("${path.module}/service-defaults.yaml"))
}

resource "kubernetes_service_account" "example-service-account" {
  metadata {
    namespace = "macrostack"
    name      = "gateway"
  }
  automount_service_account_token = true
}
