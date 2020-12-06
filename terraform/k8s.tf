terraform {
  required_providers {
    kubernetes = {
      source = "hashicorp/kubernetes"
    }
  }
}

provider "kubernetes" {
  load_config_file = false

  host     = google_container_cluster.primary.endpoint
  username = var.gke_username
  password = var.gke_password

  client_certificate     = base64decode(google_container_cluster.primary.master_auth.0.client_certificate)
  client_key             = base64decode(google_container_cluster.primary.master_auth.0.client_key)
  cluster_ca_certificate = base64decode(google_container_cluster.primary.master_auth.0.cluster_ca_certificate)
}

resource "kubernetes_deployment" "code_executor" {
  metadata {
    name = "code-executor-deploy"
    labels = {
      "app" = "code-executor"
    }
  }

  spec {
    replicas = 3
    selector {
      match_labels = {
        "app" = "code-executor"
      }
    }

    template {
      metadata {
        labels = {
          "app" = "code-executor"
        }
      }

      spec {
        container {
          image = "gcr.io/cloud-executor/code-executor:0.1.3"
          name  = "code-executor"

          port {
            container_port = 80
          }

          resources {
            limits {
              cpu = "0.7"
            }

            requests {
              cpu = "250m"
            }
          }
        }
      }
    }
  }
}

resource "kubernetes_service" "code_executor" {
  metadata {
    name = "code-executor-svc"
  }

  spec {
    selector = {
      "app" = "code-executor"
    }

    port {
      port        = 80
      target_port = 80
    }

    type = "LoadBalancer"
  }
}

resource "kubernetes_horizontal_pod_autoscaler" "code_executor" {
  metadata {
    name = "code-executor-scaler"
  }

  spec {
    scale_target_ref {
      kind = "Deployment"
      name = "code-executor-deploy"
    }

    min_replicas                      = 1
    max_replicas                      = 10
    target_cpu_utilization_percentage = 70
  }
}

output "loadbalancer_ip" {
  value = kubernetes_service.code_executor.load_balancer_ingress[0].ip
}
