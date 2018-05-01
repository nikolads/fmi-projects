#ifndef SIR_SIMUL_RESULT_H
#define SIR_SIMUL_RESULT_H

#include <vector>

namespace sir {

struct DataPoint {
    DataPoint();
    DataPoint(double _S, double _I, double _R, double _time = 0.0);

    double S;
    double I;
    double R;
    double time;
};

class SimulResult {
public:
    SimulResult();
    SimulResult(std::vector<DataPoint>&& points);

    double error(const SimulResult& other) const;

public:
    std::vector<DataPoint> points;
};

}

#endif
