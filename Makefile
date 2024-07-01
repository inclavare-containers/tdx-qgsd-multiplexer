IMAGE_REF := al3-tdx-qpl

.PHONY: image
image:
	docker build -t $(IMAGE_REF) .
