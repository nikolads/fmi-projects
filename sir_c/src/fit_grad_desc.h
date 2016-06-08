#ifndef FIT_GRADIENT_DESCEND_H
#define FIT_GRADIENT_DESCEND_H

#include "model.h"
#include <limits>
#include <cstdio>

namespace sir {

class GradientDesc {
public:
    GradientDesc(const SimulResult& _target, double _sim_time);
    std::pair<double, double> fit(double init_beta, double init_alpha, double init_step);

private:
    double error(const SimulResult& curr_res) const;
    double cost(double beta, double alpha) const;
    std::pair<double, double> cost_derivative(double beta, double alpha) const;

private:
    const SimulResult& target;
    double sim_time;

    double precision;
};

}

#endif
