.PRECIOUS : test/%.out
%: test/%.out
	@diff test/$*.out test/$*.want | tee test/$*.diff

test/%.out: src/bin/day%.rs test/%.in test/%.want
	cargo run --bin day$* < test/$*.in > test/$*.out

.PHONY: clean
clean:
	$(RM) test/*.out test/*.diff
