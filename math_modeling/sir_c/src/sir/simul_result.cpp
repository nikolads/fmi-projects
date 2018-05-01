#include "simul_result.h"

namespace sir {

DataPoint::DataPoint():
    S(0.0),
    I(0.0),
    R(0.0),
    time(0.0)
{
}

DataPoint::DataPoint(double _S, double _I, double _R, double _time):
    S(_S),
    I(_I),
    R(_R),
    time(_time)
{
}

SimulResult::SimulResult():
    points()
{
}

SimulResult::SimulResult(std::vector<DataPoint>&& _points):
    points(_points)
{
}

double SimulResult::error(const SimulResult& other) const {
    int len = std::min(this->points.size(), other.points.size());
    double err = 0.0;

    for (int i = 0; i < len; i++) {
        double delta = this->points[i].I - other.points[i].I;
        err += delta * delta;

        delta = this->points[i].S - other.points[i].S;
        err += delta * delta;
    }

    return err;
}

}
