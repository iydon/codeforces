CARGO = cargo
PYTHON = python


.PHONY: run test expand readme analysis

run:
	$(CARGO) run --bin codeforces

test:
	$(CARGO) test

expand:
	$(CARGO) expand --bin codeforces > src/bin/expanded.rs

readme:
	$(PYTHON) scripts/README.py

analysis:
	# https://github.com/XAMPPRocky/tokei
	$(PYTHON) scripts/analysis.py > cache.rs
	tokei scripts/*.py *.md Makefile cache.rs
	rm cache.rs
