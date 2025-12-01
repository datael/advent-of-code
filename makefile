clippy:
	cargo clippy --all -- -W clippy::pedantic -A clippy::needless_pass_by_value -A clippy::module_name_repetitions -A clippy::explicit_iter_loop -A clippy::wildcard_imports -A clippy::missing_errors_doc
