CARGO = cargo
PYTHON = python


.PHONY: run expand readme

run:
	$(CARGO) run

expand:
	$(CARGO) expand

readme:
	$(PYTHON) README.py
