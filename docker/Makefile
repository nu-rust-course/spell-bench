DOCKERFILES = $(wildcard *.df)
IMAGES      = $(DOCKERFILES:%.df=%)
TARGETS     = $(DOCKERFILES:%.df=.%.built)

default: $(TARGETS)

.%.built: %.df
	docker build -t $* -f $< .
	touch $@

.PHONY: clean
clean:
	$(RM) $(TARGETS)
	for img in $(IMAGES); do docker rmi $$img && true; done
	docker image prune -f
