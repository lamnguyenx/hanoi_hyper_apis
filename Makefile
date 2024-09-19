.PHONY: all py go rust cpp swift

all: py go rust cpp swift

python:
	export PYTHONPATH="$$(pwd)/projects/hanoi_learn_python/src" && \
	cd projects/hanoi_learn_python && \
	./main.py | pygmentize \
		2>&1 | sed 's/^/py    | /'

go:
	cd projects/hanoi_learn_go/src && \
	go build -o .build/main main.go | pygmentize && \
	.build/main \
		2>&1 | pygmentize | sed 's/^/go    | /'

rust:
	cd projects/hanoi_learn_rust && \
	cargo build --quiet --target-dir=exp | pygmentize && \
	exp/debug/hanoi_learn-rust \
		2>&1 | pygmentize | sed 's/^/rust  | /'

cpp:
	cd projects/hanoi_learn_cpp && \
	mkdir -p build && cd build && \
	(cmake .. && make) 2>&1 | pygmentize | grep -i -A 100 -B 2 error || \
	./out \
		2>&1 | pygmentize | sed 's/^/cpp   | /'

swift:
	cd projects/hanoi_learn_swift && \
	(swift build) 2>&1 | pygmentize | grep -i -A 100 -B 2 error || \
	$$(swift build --show-bin-path)/hanoi_learn_swift \
		2>&1 | pygmentize | sed 's/^/swift | /'

java:
	cd projects/hanoi_learn_java && \
	java src/Main.java \
		2>&1 | pygmentize | sed 's/^/java  | /'



master-swift:
	cd web-service/master/master-swift && \
	swift run
