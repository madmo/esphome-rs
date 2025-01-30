fn main() {
	protobuf_codegen::Codegen::new()
		.protoc()
		.includes(&["src"])
		.input("src/api.proto")
		.input("src/api_options.proto")
		.cargo_out_dir("protos")
		.run_from_script();
}
