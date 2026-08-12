all:
	cc string.c -L string/target/release -l:libstring.a
clean:
	rm -rf a.out
