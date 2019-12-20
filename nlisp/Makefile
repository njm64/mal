

OUTDIR=target/debug

$(OUTDIR)/step0_repl: src/step0_repl.rs
	cargo build --bin step0_repl

$(OUTDIR)/step1_read_print: src/step1_read_print.rs src/reader.rs src/printer.rs src/types.rs
	cargo build --bin step1_read_print
