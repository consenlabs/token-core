SHELL := /bin/bash

.PHONY: test

help: ##@other show help
	@perl -e '$(HELP_FUN)' $(MAKEFILE_LIST)

# This is a code for automatic help generator.
# To add new item into help output, simply add comments
# starting with '##'. To add category, use @category.
GREEN  := $(shell tput -Txterm setaf 2)
WHITE  := $(shell tput -Txterm setaf 7)
YELLOW := $(shell tput -Txterm setaf 3)
RESET  := $(shell tput -Txterm sgr0)
HELP_FUN = \
	%help; \
	while(<>) { push @{$$help{$$2 // 'options'}}, [$$1, $$3] if /^([a-zA-Z0-9\-]+)\s*:.*\#\#(?:@([a-zA-Z0-9\-]+))?\s(.*)$$/ }; \
	print "Usage: make [target]\n\n"; \
	for (sort keys %help) { \
		print "${WHITE}$$_:${RESET}\n"; \
		for (@{$$help{$$_}}) { \
			$$sep = " " x (32 - length $$_->[0]); \
			print "  ${YELLOW}$$_->[0]${RESET}$$sep${GREEN}$$_->[1]${RESET}\n"; \
		}; \
		print "\n"; \
	}

build-ios-example: ##@build generate the iOS compiled file
	./tools/ios-example-build.sh

build-ios-rn-example: ##@build generate the iOS RN compiled file
	sh ./tools/ios-rn-example-build.sh

build-android-rn-example: ##@build generate the Android RN compiled file
	sh ./tools/android-rn-example-build.sh

internal-release-android:
	./tools/android-token-v2-build.sh
    
internal-release-ios: ##@build generate the Android RN compiled file
	sh ./tools/ios-framework-build.sh
	sh ./tools/ios-internal-release.sh $(VER)

e2e: ##@test run e2e test
	(cd examples/RN && yarn)
	(cd examples/RN/ios && pod install)
	(cd examples/RN && yarn pbjs)
	(cd examples/RN && yarn e2e)

