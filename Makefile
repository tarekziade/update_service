COMPONENT=$(shell basename $$(pwd))

.PHONY: clean dist zip deps

default: clean deps dist zip;

deps:
	test -d lib || mkdir lib
	# Install python's dependencies.

dist: deps
	test -d dist || mkdir dist
	# Build rust library
	cargo build --release
	# Move the library to CWD
	cp target/release/libupdate_service.* dist/
	# Copy the source and library to dist
	cp *.py dist/

zip: dist
# Package the dist folder into a zip to deploy
	cd dist && zip -q -r ../$(COMPONENT).zip .

clean:
	cargo clean
	rm -rf *.so $(COMPONENT).zip lib/* dist/*

test:
	cargo test
