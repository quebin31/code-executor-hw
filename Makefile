version := $(shell cat Cargo.toml | grep -E "^version = .*$$" | cut -d= -f2 | sed 's/[" ]//g')
gcp_pid := "cloud-executor"
gcp_region := "us-central1"
gcp_registry := "gcr.io/$(gcp_pid)"
aws_registry := "768088100333.dkr.ecr.us-east-1.amazonaws.com"

include .env

# =================================================
# Build, tag and push the image to a registry
# =================================================
build-image:
	@docker build -t code-executor:$(version) .

tag-image-gcr:
	@docker tag code-executor:$(version) $(gcp_registry)/code-executor:$(version)

push-image-gcr: 
	@docker push $(gcp_registry)/code-executor:$(version)

tag-image-ecr:
	@docker tag code-executor:$(version) $(aws_registry)/code-executor:$(version)

push-image-ecr:
	@docker push $(aws_registry)/code-executor:$(version)


# =================================================
# Terraform interface
# =================================================
terra-init:
	@cd terraform; terraform init 
	
terra-plan:
	@cd terraform; terraform plan

terra-apply:
	@cd terraform; terraform apply

terra-refresh:
	@cd terraform; terraform refresh

terra-destroy:	
	@cd terraform; terraform destroy

# =================================================
# Apply and delete deployment and service (manual)
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
