all:
	rustc repl.rs

arith:
	rustc arith.rs

clean:
	if [ -f repl ]; then rm repl; fi
	if [ -f arith ]; then rm arith; fi

repl:
	rustc repl.rs