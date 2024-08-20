.PHONY: get_ver

get_ver:
	curl -v -X GET http://localhost:8888/version

get_arch:
	curl -v -X GET http://localhost:8888/arches/1

# post_arch:
# 	curl -vvv -X POST http://localhost:8888/arches -H "Content-Type: application/json" -d '{"name": "aarch64"}'


