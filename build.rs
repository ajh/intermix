extern crate gcc;

fn main() {
	gcc::Config::new()
		.file("vendor/libtsm/src/tsm_screen.c")
		.file("vendor/libtsm/src/tsm_unicode.c")
		.file("vendor/libtsm/src/tsm_vte.c")
		.file("vendor/libtsm/src/tsm_vte_charsets.c")
		.include("vendor/libtsm")
		.compile("libtsm.a");
}
