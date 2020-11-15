version := $(shell cat Cargo.toml | grep -E "^version = .*$$" | cut -d= -f2 | sed 's/[" ]//g')
gcp_pid := "web-searcher-293217"
gcp_zone := "us-central1-f"
gcp_registry := "gcr.io/$(gcp_pid)"

build-image:
	@docker build -t code-executor:$(version) .

tag-image:
	@docker tag code-executor:$(version) $(gcp_registry)/code-executor:$(version)

push-image: 
	@docker push $(gcp_registry)/code-executor:$(version)

create-cluster:
	@gcloud container clusters create code-executor-cluster

delete-cluster: 
	@gcloud container clusters delete code-executor-cluster

deploy-service:
	@kubectl apply -f deploy.yaml
	@kubectl apply -f service.yaml

undeploy-service:
	@kubectl delete service code-executor-service



