CARGO = cargo
PYTHON = python


.PHONY: run test expand readme analysis

analysis:
	# https://github.com/XAMPPRocky/tokei
	$(PYTHON) analysis.py > cache.rs
	tokei *.py *.md Makefile cache.rs
	rm cache.rs

run:
	$(CARGO) run --bin codeforces

test:
	$(CARGO) test

expand:
	$(CARGO) expand --bin codeforces > src/bin/expanded.rs

readme:
	$(PYTHON) README.py
