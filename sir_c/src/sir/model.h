#ifndef SIR_MODEL_H
#define SIR_MODEL_H

#include "simul_result.h"
#include <vector>
#include <cmath>

namespace sir {

/// Analytical simulation of the SIR model
class Model {
public:
    Model(DataPoint _initial_cond, double _infect_prob, double _recovery_rate);

    /// Make a simulation and return data for time period [0; sim_time]
    SimulResult simulate(double sim_time);

public:
    DataPoint initial_cond;
    double infect_prob;
    double recovery_rate;

    double result_step;
    double sim_step;
};

}

#endif
