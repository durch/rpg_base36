EXTENSION    = rpg_base36
EXTVERSION   = 0.0.1

DATA         = sql/$(EXTENSION)--$(EXTVERSION).sql
REGRESS      = rpg_base36_tests
PG_CONFIG    = pg_config

all: sql/$(EXTENSION)--$(EXTVERSION).sql

sql/$(EXTENSION)--$(EXTVERSION).sql: sql/$(EXTENSION).sql
	cp $< $@

EXTRA_CLEAN = sql/$(EXTENSION)--$(EXTVERSION).sql

# Note that `MODULES = rpg_base36` implies a dependency on `rpg_base36.so`.
MODULES      = rpg_base36
PGXX        := $(shell utils/get_version.sh)
RS          := $(shell which cargo >/dev/null && echo yes || echo no)

ifeq ($(shell uname -s),Darwin)
    LINK_FLAGS   = -C link-args='-Wl,-undefined,dynamic_lookup'
endif

.PHONY: rpg_base36.so
rpg_base36.so:
	cargo rustc --release -- $(LINK_FLAGS)
	cp target/release/librpg_base36.* $@

.PHONY: cargoclean
cargoclean:
	find . -name Cargo.lock -exec rm {} \;
	cargo clean

PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)

clean: cargoclean

all: rpg_base36.so
