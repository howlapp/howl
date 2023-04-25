terraform {
  required_version = ">= 1.4"
  required_providers {
    helm = {
      source  = "hashicorp/helm"
      version = "2.9.0"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "2.19.0"
    }
  }
}

provider "helm" {
  kubernetes {
    host = var.kubernetes_host
  }
}

provider "kubernetes" {
  host = var.kubernetes_host
}

module "postgres" {
  source            = "./modules/postgres"
  postgres_version  = var.postgres_version
  postgres_password = var.postgres_password
}

module "nginx" {
  source = "./modules/nginx"
}

module "example-service" {
  source     = "./modules/example-service"
  depends_on = [kubernetes_secret.ghcr-token]
  commit     = var.commit
}

module "gateway" {
  source = "./modules/gateway"
  depends_on = [
    kubernetes_secret.ghcr-token
  ]
  commit = var.commit
}

resource "kubernetes_secret" "ghcr-token" {
  type = "kubernetes.io/dockerconfigjson"
  metadata {
    namespace = "macrostack"
    name      = "ghcr-token"
  }
  data = {
    ".dockerconfigjson" = jsonencode({
      auths = {
        "ghcr.io" = {
          "auth" = base64encode("${var.ghcr_username}:${var.ghcr_token}")
        }
      }
    })
  }
}
