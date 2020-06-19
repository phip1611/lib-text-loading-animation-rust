# link against name without "lib" prefix and without ".so" suffix
RUST_LIB_NAME=text_loading_animation
RUST_LIB_DIR=../target/release
RUST_LIB=$(RUST_LIB_DIR)/$(RUST_LIB_NAME).so

main: main.c | $(RUST_LIB)
	# link against name without "lib" prefix and without ".so" suffix
	# file is called "libtext_loading_animation.so"
	# it's important that the library (-l) comes after the c file!!
	# see: https://stackoverflow.com/a/12748882/2891595
	gcc -Wall -Werror -ggdb -L$(RUST_LIB_DIR) -o $@ $< -l$(RUST_LIB_NAME)

$(RUST_LIB):
	cd ..
	cargo build --release

.PHONY: clean

clean:
	rm -f ./main
