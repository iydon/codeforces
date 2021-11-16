CARGO = cargo
PYTHON = python


.PHONY: run expand readme

run:
	$(CARGO) run --bin codeforces

expand:
	$(CARGO) expand --bin codeforces > src/bin/expanded.rs

readme:
	$(PYTHON) README.py
