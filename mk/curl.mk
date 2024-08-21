ARCH_ID = 100

.PHONY: get_ver

get_ver:
	curl -v -X GET http://localhost:8888/version

get_arch:
	curl -v -X GET http://localhost:8888/arches/$(ARCH_ID)

post_arch:
	curl -vvv -X POST http://localhost:8888/arches -H "Content-Type: application/json" -d '{"name": "aarch64"}'

update_arch:
	curl -vvv -X PUT http://localhost:8888/arches/$(ARCH_ID) -H "Content-Type: application/json" -d '{"name": "aarch64", description: "arm 64"}'

del_arch:
	curl -v -X DELETE http://localhost:8888/arches/$(ARCH_ID)