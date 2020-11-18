version := $(shell cat Cargo.toml | grep -E "^version = .*$$" | cut -d= -f2 | sed 's/[" ]//g')
gcp_pid := "code-executor-cloud"
gcp_region := "southamerica-east1"
gcp_regzone := "southamerica-east1-a"
gcp_registry := "gcr.io/$(gcp_pid)"

build-image:
	@docker build -t code-executor:$(version) .

tag-image:
	@docker tag code-executor:$(version) $(gcp_registry)/code-executor:$(version)

push-image: 
	@docker push $(gcp_registry)/code-executor:$(version)

create-cluster:
	@gcloud config set project $(gcp_pid)
	@gcloud config set compute/zone $(gcp_regzone)
	@gcloud container clusters create code-executor-cluster

delete-cluster: 
	@gcloud config set project $(gcp_pid)
	@gcloud config set compute/zone $(gcp_regzone)
	@gcloud container clusters delete code-executor-cluster

reserve-ip:
	@gcloud config set project $(gcp_pid)
	@gcloud compute addresses create code-executor-ip --region $(gcp_region)

unreserve-ip:
	@gcloud config set project $(gcp_pid)
	@gcloud compute addresses delete code-executor-ip --region $(gcp_region)

get-reserved-ip:
	@gcloud config set project $(gcp_pid)
	@gcloud compute addresses describe code-executor-ip --region $(gcp_region)

apply-deploy:
	@kubectl apply -f deploy.yaml

apply-service:
	@kubectl apply -f service.yaml

undeploy-service:
	@kubectl delete service code-executor-service



