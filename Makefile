CARGO = cargo
PYTHON = python


.PHONY: run test expand readme

run:
	$(CARGO) run --bin codeforces

test:
	$(CARGO) test

expand:
	$(CARGO) expand --bin codeforces > src/bin/expanded.rs

readme:
	$(PYTHON) README.py
