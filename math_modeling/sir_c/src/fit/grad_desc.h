#ifndef FIT_GRADIENT_DESCEND_H
#define FIT_GRADIENT_DESCEND_H

#include "sir/sir.h"
#include <limits>
#include <cstdio>

class GradientDesc {
public:
    GradientDesc(const sir::SimulResult& _target, double _sim_time);
    std::pair<double, double> fit(double init_beta, double init_alpha, double init_step);

private:
    double cost(double beta, double alpha) const;
    std::pair<double, double> cost_derivative(double beta, double alpha) const;

public:
    double precision;

private:
    const sir::SimulResult& target;
    double sim_time;
};

#endif
