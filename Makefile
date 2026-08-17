dev:
	export $(shell cat .env | xargs) && cargo leptos watch
