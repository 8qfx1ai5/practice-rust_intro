# main app phonies
.PHONY: compile run
# -------------------------------------------------------------------------

# run the rust compiler
compile:
	@-rm -r output/* 2> /dev/null
	rustc --out-dir output code/start.rs

run:
	output/start
