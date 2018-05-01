#ifndef FIT_SIMULATED_ANNEALING_H
#define FIT_SIMULATED_ANNEALING_H

#define USE_MATH_DEFINES
#include <cmath>

#include "sir/sir.h"
#include <cstdlib>
#include <limits>
#include <mutex>
#include <thread>

#define USE_COLOR 0

class SimulAnnealing {
public:
    SimulAnnealing(const sir::SimulResult& _target, double _sim_time);

    void start(double init_beta, double init_alpha, double step_beta, double step_alpha);

private:
    double cost(double beta, double alpha) const;
    double acceptance(double curr_cost, double new_cost) const;

public:
    int nthreads;
    double init_temp;
    double cooling_rate;

private:
    const sir::SimulResult& target;
    double sim_time;
};

#endif
