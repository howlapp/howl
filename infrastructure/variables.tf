variable "kubernetes_host" {
  type        = string
  description = "The host of the Kubernetes cluster"
}

variable "postgres_version" {
  type        = string
  description = "Postgres version"
  default     = "15.2.0"
}

variable "postgres_password" {
  type        = string
  description = "The password for the default user"
  default     = "postgres"
}

variable "ghcr_username" {
  description = "The username to access the GitHub Container Registry"
  type        = string
}

variable "ghcr_token" {
  description = "The token to access the GitHub Container Registry"
  type        = string
}

variable "commit" {
  description = "The hash of the commit to deploy"
  type        = string
}
