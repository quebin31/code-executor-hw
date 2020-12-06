variable "project_id" {
  description = "Project Id"
}

variable "region" {
  description = "Region"
}

provider "google" {
  project = var.project_id
  region  = var.region
}

resource "google_compute_network" "vpc" {
  name                    = "${var.project_id}-vpc"
  auto_create_subnetworks = false
}

resource "google_compute_subnetwork" "subnet" {
  name          = "${var.project_id}-subnet"
  region        = var.region
  network       = google_compute_network.vpc.name
  ip_cidr_range = "10.10.0.0/24"
}

resource "google_compute_address" "static" {
  name         = "${var.project_id}-address"
  address_type = "EXTERNAL"
}

output "region" {
  value       = var.region
  description = "Region"
}
