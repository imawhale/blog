export RUST_BACKTRACE := "1"

watch:
	cargo watch --ignore 'www/*' --clear --exec 'run render'

render:
	cargo run render

serve:
	cd www && netlify dev

open:
	open 'http://127.0.0.1:8888/blog'

browse:
	open 'http://rodarmor.com/blog'

deploy:
	git push gitlab

deps:
	curl https://raw.githubusercontent.com/necolas/normalize.css/master/normalize.css | \
		grep -v '^/\*! normalize.css' > in/normalize.css

post SLUG:
	cargo run post {{SLUG}}
