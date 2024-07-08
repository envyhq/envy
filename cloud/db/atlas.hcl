variable "turso_token" {
  type    = string
  default = getenv("TURSO_TOKEN")
}

variable "cloud_token" {
  type    = string
  default = getenv("ATLAS_TOKEN")
}

atlas {
  cloud {
    token = var.cloud_token
  }
}

env "development" {
  url     = "libsql+wss://envy-development-lbennett-stacki.turso.io?authToken=${var.turso_token}"
  exclude = ["_litestream*"]
  migration {
    dir = "atlas://envy"
  }
}
