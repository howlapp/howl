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
