
shader_sources := $(wildcard shaders/src/*)
compiled_names := $(shader_sources:shaders/src/%=shaders/%)

compiler := glslangValidator
compiler_args := -V

all: $(compiled_names)

clean:
	@rm -f --verbose $(compiled_names)

.PHONY: all-glslc, clean

shaders/%: shaders/src/%
	@echo "-> $*:"
	$(compiler) $(compiler_args) -o shaders/$* shaders/src/$*
	@echo ""