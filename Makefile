all:
	mpic++ -O2 -o circuitSatisfiability circuitSatisfiability.cc

run: circuitSatisfiability
	./circuitSatisfiability
