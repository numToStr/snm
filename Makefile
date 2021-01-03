.PHONY := release

release:
	@test $(VERSION)
	@echo ">> VERSION = $(VERSION)"
	@echo ">> Updating Cargo.toml"
	sed -i -e 's/^version = .*/version = "$(VERSION)"/g' Cargo.toml
	@# This will update the Cargo.toml
	@cargo update
	@echo ">> Release"
	@# You need git-release from tj/git-extras
	git release v$(VERSION)
