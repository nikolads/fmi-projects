#ifndef SIR_MODEL_H
#define SIR_MODEL_H

#include <vector>
#include <cmath>

namespace sir {

struct DataPoint {
    DataPoint();
    DataPoint(double _S, double _I, double _R, double _time = 0.0);

    double S;
    double I;
    double R;
    double time;
};

typedef std::vector<DataPoint> SimulResult;

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
