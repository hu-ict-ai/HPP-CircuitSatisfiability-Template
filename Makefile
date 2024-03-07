all: circuitSatisfiability

circuitSatisfiability: circuitSatisfiability.cc
	mpic++ -O2 -o circuitSatisfiability circuitSatisfiability.cc

run: circuitSatisfiability
	./circuitSatisfiability
