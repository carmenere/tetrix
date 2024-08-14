.PHONY: get_ver

get_ver:
	curl -v -X GET http://localhost:8888/version

# post_item:
# 	curl -vvv -X POST http://localhost:8888/items -H "Content-Type: application/json" -d '{"item": "foo"}'

# get_item:
# 	curl -v -X GET http://localhost:8888/items/1
