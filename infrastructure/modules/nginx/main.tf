resource "kubernetes_manifest" "nginx-service" {
  manifest = yamldecode(file("${path.module}/service.yaml"))
}

resource "kubernetes_manifest" "nginx-deployment" {
  manifest = yamldecode(file("${path.module}/deployment.yaml"))
}

resource "kubernetes_manifest" "nginx-service-defaults" {
  manifest = yamldecode(file("${path.module}/service-defaults.yaml"))
}

resource "kubernetes_manifest" "nginx-gateway" {
  manifest = yamldecode(file("${path.module}/gateway.yaml"))
}

resource "kubernetes_manifest" "nginx-routes" {
  manifest = yamldecode(file("${path.module}/routes.yaml"))
}

resource "kubernetes_service_account" "nginx-service-account" {
  metadata {
    namespace = "macrostack"
    name      = "nginx"
  }
  automount_service_account_token = true
}

resource "kubernetes_manifest" "nginx-config" {
  manifest = yamldecode(file("${path.module}/config.yaml"))
}

resource "kubernetes_manifest" "nginx-reference-grant" {
  manifest = yamldecode(file("${path.module}/referencegrant.yaml"))
}
