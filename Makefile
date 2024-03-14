ifndef NP
override NP = 4
endif

all: circuitSatisfiability

circuitSatisfiability: circuitSatisfiability.cc
	mpic++ -O2 -o circuitSatisfiability circuitSatisfiability.cc

run: circuitSatisfiability
	mpirun -oversubscribe -np $(NP) ./circuitSatisfiability
