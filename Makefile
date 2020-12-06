version := $(shell cat Cargo.toml | grep -E "^version = .*$$" | cut -d= -f2 | sed 's/[" ]//g')
gcp_pid := "cloud-executor"
gcp_region := "us-central1"
gcp_registry := "gcr.io/$(gcp_pid)"

include .env

# =================================================
# Build, tag and push the image to a registry
# =================================================
build-image:
	@docker build -t code-executor:$(version) .

tag-image:
	@docker tag code-executor:$(version) $(gcp_registry)/code-executor:$(version)

push-image: 
	@docker push $(gcp_registry)/code-executor:$(version)

# =================================================
# Terraform interface
# =================================================
terra-init:
	@cd terraform; terraform init 
	
terra-plan:
	@cd terraform; terraform plan

terra-apply:
	@cd terraform; terraform apply
	@cd terraform; gcloud container clusters get-credentials \
		$(shell terraform output cluster) \
		--refion $(shell terraform output region)

terra-destroy:	
	@terraform destroyt terraform

# =================================================
# Reserve and unreserve static ip with `gcloud`
# =================================================
reserve-ip:
	@cd terraform; gcloud compute addresses create code-executor-ip \
		--project $(gcp_pid) \
		--region $(shell terraform output region) \
		--network $(shell terraform output network) \
		--subnet $(shell terraform output subnet)

unreserve-ip:
	@cd terraform; gcloud compute addresses delete code-executor-ip \
		--project $(gcp_pid) \
		--region $(shell terraform output region) \
		--network $(shell terraform output network) \
		--subnet $(shell terraform output subnet)

get-reserved-ip:
	@cd terraform; gcloud compute addresses describe code-executor-ip \
		--project $(gcp_pid) \
		--region $(shell terraform output region) \
		--network $(shell terraform output network) \
		--subnet $(shell terraform output subnet)

# =================================================
# Apply and delete deployment and service
# =================================================
apply-deploy:
	@kubectl apply -f k8s/deployment.yaml

apply-service:
	@sed "s/\$$SERVICE_IP/$(SERVICE_IP)/" k8s/service.in.yaml > service.yaml
	@kubectl apply -f service.yaml
	@rm service.yaml

del-service:
	@kubectl delete service code-executor-service

del-deploy:
	@kubectl delete deployment code-executor-deployment
